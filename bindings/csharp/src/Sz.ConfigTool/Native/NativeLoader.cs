using System;
using System.IO;
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

        /// <summary>The runtime identifier this process should load, e.g. <c>osx-arm64</c>, or null.</summary>
        public static string? CurrentRid()
        {
            string? os = RuntimeInformation.IsOSPlatform(OSPlatform.Windows) ? "win"
                : RuntimeInformation.IsOSPlatform(OSPlatform.OSX) ? "osx"
                : RuntimeInformation.IsOSPlatform(OSPlatform.Linux) ? "linux"
                : null;
            string? arch = RuntimeInformation.ProcessArchitecture switch
            {
                Architecture.X64 => "x64",
                Architecture.Arm64 => "arm64",
                _ => null,
            };
            return os == null || arch == null ? null : os + "-" + arch;
        }

        /// <summary>The platform file name of the native library.</summary>
        public static string NativeFileName()
        {
            if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            {
                return NativeMethods.LibraryName + ".dll";
            }

            string ext = RuntimeInformation.IsOSPlatform(OSPlatform.OSX) ? ".dylib" : ".so";
            return "lib" + NativeMethods.LibraryName + ext;
        }

        /// <summary>
        /// <c>runtimes/&lt;rid&gt;/native/&lt;file&gt;</c> under <paramref name="baseDir"/>,
        /// or null when the RID is unsupported.
        /// </summary>
        public static string? BundledPath(string baseDir)
        {
            string? rid = CurrentRid();
            return rid == null ? null : Path.Combine(baseDir, "runtimes", rid, "native", NativeFileName());
        }

        private static void Install()
        {
            Assembly coreLib = typeof(object).Assembly;
            Type? nativeLibrary = coreLib.GetType("System.Runtime.InteropServices.NativeLibrary");
            Type? resolverType = coreLib.GetType("System.Runtime.InteropServices.DllImportResolver");
            if (nativeLibrary == null || resolverType == null)
            {
                return;
            }

            _tryLoad = nativeLibrary.GetMethod("TryLoad", new[] { typeof(string), typeof(IntPtr).MakeByRefType() });
            MethodInfo? setResolver = nativeLibrary.GetMethod("SetDllImportResolver", new[] { typeof(Assembly), resolverType });
            MethodInfo? resolve = typeof(NativeLoader).GetMethod(nameof(Resolve), BindingFlags.NonPublic | BindingFlags.Static);
            if (_tryLoad == null || setResolver == null || resolve == null)
            {
                return;
            }

            Delegate resolver = Delegate.CreateDelegate(resolverType, resolve);
            try
            {
                setResolver.Invoke(null, new object[] { typeof(NativeLoader).Assembly, resolver });
            }
            catch (TargetInvocationException e) when (e.InnerException is InvalidOperationException)
            {
                // The host already installed its own resolver for this
                // assembly; it takes precedence by design.
            }
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
        internal static IntPtr LoadBundled(Assembly assembly)
        {
            if (_tryLoad == null)
            {
                return IntPtr.Zero;
            }

            // Location is empty for single-file apps; use the app directory then.
            string? dir = string.IsNullOrEmpty(assembly.Location)
                ? AppContext.BaseDirectory
                : Path.GetDirectoryName(assembly.Location);
            string? path = string.IsNullOrEmpty(dir) ? null : BundledPath(dir!);
            if (path == null || !File.Exists(path))
            {
                return IntPtr.Zero;
            }

            object?[] args = { path, IntPtr.Zero };
            bool loaded = (bool)_tryLoad.Invoke(null, args)!;
            return loaded ? (IntPtr)args[1]! : IntPtr.Zero;
        }
    }
}
