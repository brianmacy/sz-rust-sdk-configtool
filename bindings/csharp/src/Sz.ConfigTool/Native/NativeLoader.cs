using System;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Runtime.InteropServices;

namespace Sz.ConfigTool.Native
{
    /// <summary>
    /// Installs a <c>DllImport</c> resolver for <see cref="NativeMethods.LibraryName"/>
    /// that first loads <c>runtimes/&lt;rid&gt;/native/&lt;file&gt;</c> next to
    /// this assembly, then falls back to the runtime's default loader.
    /// </summary>
    /// <remarks>
    /// The library targets netstandard2.0, where <c>System.Runtime.InteropServices.NativeLibrary</c>
    /// (.NET Core 3.0+) is not referenceable, so it is bound by reflection. On
    /// runtimes without it (.NET Framework) or when the host already installed a
    /// resolver for this assembly, the default loader is used unchanged.
    /// </remarks>
    internal static class NativeLoader
    {
        private static readonly object Gate = new object();
        private static bool _installed;
        private static MethodInfo? _tryLoad;

        /// <summary>Install the resolver once (idempotent, thread-safe).</summary>
        public static void EnsureInstalled()
        {
            lock (Gate)
            {
                if (_installed)
                {
                    return;
                }

                _installed = true;
                Install();
            }
        }

        // The supported host OSes and their RID prefix (first match wins).
        private static readonly (OSPlatform Platform, string Rid)[] KnownOs =
        {
            (OSPlatform.Windows, "win"),
            (OSPlatform.OSX, "osx"),
            (OSPlatform.Linux, "linux"),
        };

        /// <summary>The host OS's RID prefix (<c>win</c>, <c>osx</c>, <c>linux</c>), or null.</summary>
        internal static string? CurrentOs() =>
            KnownOs.FirstOrDefault(os => RuntimeInformation.IsOSPlatform(os.Platform)).Rid;

        /// <summary>The runtime identifier this process should load, e.g. <c>osx-arm64</c>, or null.</summary>
        public static string? CurrentRid() => Rid(CurrentOs(), RuntimeInformation.ProcessArchitecture);

        /// <summary>
        /// The RID for an OS prefix and process architecture; null for an
        /// unsupported OS (null) or architecture (anything but x64/arm64).
        /// </summary>
        internal static string? Rid(string? os, Architecture architecture)
        {
            string? arch = architecture switch
            {
                Architecture.X64 => "x64",
                Architecture.Arm64 => "arm64",
                _ => null,
            };
            return os == null || arch == null ? null : os + "-" + arch;
        }

        /// <summary>The platform file name of the native library.</summary>
        public static string NativeFileName() => FileName(CurrentOs());

        /// <summary>The native library's file name on OS <paramref name="os"/> (a RID prefix).</summary>
        internal static string FileName(string? os) => os switch
        {
            "win" => NativeMethods.LibraryName + ".dll",
            "osx" => "lib" + NativeMethods.LibraryName + ".dylib",
            _ => "lib" + NativeMethods.LibraryName + ".so",
        };

        /// <summary>
        /// <c>runtimes/&lt;rid&gt;/native/&lt;file&gt;</c> under <paramref name="baseDir"/>,
        /// or null when the RID is unsupported.
        /// </summary>
        public static string? BundledPath(string baseDir) => BundledPath(baseDir, CurrentRid(), NativeFileName());

        /// <summary>
        /// <c>runtimes/&lt;rid&gt;/native/&lt;fileName&gt;</c> under <paramref name="baseDir"/>,
        /// or null for an unsupported (null) <paramref name="rid"/>.
        /// </summary>
        internal static string? BundledPath(string baseDir, string? rid, string fileName) =>
            rid == null ? null : Path.Combine(baseDir, "runtimes", rid, "native", fileName);

        private static void Install()
        {
            Assembly coreLib = typeof(object).Assembly;
            _tryLoad = Bind(
                coreLib.GetType("System.Runtime.InteropServices.NativeLibrary"),
                coreLib.GetType("System.Runtime.InteropServices.DllImportResolver"),
                typeof(NativeLoader).Assembly);
        }

        /// <summary>
        /// Register <see cref="Resolve"/> for <paramref name="assembly"/> through
        /// the reflected <c>NativeLibrary</c> / <c>DllImportResolver</c> types and
        /// return <c>NativeLibrary.TryLoad</c>; null (the default loader stays
        /// in charge) when the runtime lacks them (.NET Framework) or they lack
        /// the expected members. A resolver the host already registered for the
        /// assembly takes precedence by design.
        /// </summary>
        internal static MethodInfo? Bind(Type? nativeLibrary, Type? resolverType, Assembly assembly)
        {
            if (nativeLibrary == null || resolverType == null)
            {
                return null;
            }

            MethodInfo? tryLoad = nativeLibrary.GetMethod("TryLoad", new[] { typeof(string), typeof(IntPtr).MakeByRefType() });
            MethodInfo? setResolver = nativeLibrary.GetMethod("SetDllImportResolver", new[] { typeof(Assembly), resolverType });
            if (tryLoad == null || setResolver == null)
            {
                return null;
            }

            Func<string, Assembly, DllImportSearchPath?, IntPtr> resolve = Resolve;
            Delegate resolver = Delegate.CreateDelegate(resolverType, resolve.Method);
            try
            {
                setResolver.Invoke(null, new object[] { assembly, resolver });
            }
            catch (TargetInvocationException e) when (e.InnerException is InvalidOperationException)
            {
                // The host already installed its own resolver for this
                // assembly; it takes precedence by design.
            }

            return tryLoad;
        }

        // Signature matches System.Runtime.InteropServices.DllImportResolver.
        private static IntPtr Resolve(string libraryName, Assembly assembly, DllImportSearchPath? searchPath)
        {
            return libraryName == NativeMethods.LibraryName ? LoadBundled(assembly) : IntPtr.Zero;
        }

        /// <summary>
        /// Load the bundled <c>runtimes/&lt;rid&gt;/native</c> library next to
        /// <paramref name="assembly"/>; <see cref="IntPtr.Zero"/> when absent or
        /// when <c>NativeLibrary</c> is unavailable.
        /// </summary>
        internal static IntPtr LoadBundled(Assembly assembly) => LoadBundled(assembly, _tryLoad);

        /// <summary>
        /// <see cref="LoadBundled(Assembly)"/> with the bound
        /// <c>NativeLibrary.TryLoad</c> (null on runtimes without it).
        /// </summary>
        internal static IntPtr LoadBundled(Assembly assembly, MethodInfo? tryLoad)
        {
            string? path = BundledPath(DirectoryOf(assembly.Location));
            if (tryLoad == null || path == null || !File.Exists(path))
            {
                return IntPtr.Zero;
            }

            object?[] args = { path, IntPtr.Zero };
            bool loaded = (bool)tryLoad.Invoke(null, args)!;
            return loaded ? (IntPtr)args[1]! : IntPtr.Zero;
        }

        /// <summary>
        /// The directory of an assembly <paramref name="location"/>: the app
        /// directory when it is empty (single-file apps) or has no parent.
        /// </summary>
        internal static string DirectoryOf(string? location)
        {
            string? dir = string.IsNullOrEmpty(location) ? null : Path.GetDirectoryName(location);
            return string.IsNullOrEmpty(dir) ? AppContext.BaseDirectory : dir!;
        }
    }
}
