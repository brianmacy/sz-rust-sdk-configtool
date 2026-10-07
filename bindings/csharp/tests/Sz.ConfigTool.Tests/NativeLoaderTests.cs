using System;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Text.RegularExpressions;
using Sz.ConfigTool.Native;
using Sz.ConfigTool.Tests.Support;
using Xunit;

namespace Sz.ConfigTool.Tests
{
    /// <summary>The runtimes/&lt;rid&gt;/native resolver and library identity.</summary>
    public class NativeLoaderTests
    {
        private static string AssemblyDir => Path.GetDirectoryName(typeof(SzConfigTool).Assembly.Location)!;

        [Fact]
        public void Native_library_is_loaded_from_runtimes_rid_native()
        {
            string expected = NativeLoader.BundledPath(AssemblyDir)!;
            Assert.True(File.Exists(expected), expected);
            Assert.Equal(Path.Combine("runtimes", NativeLoader.CurrentRid()!, "native", NativeLoader.NativeFileName()),
                Path.GetRelativePath(AssemblyDir, expected));

            // The only copy reachable from the app directory is the bundled one,
            // and the resolver loads it.
            Assert.False(File.Exists(Path.Combine(AssemblyDir, NativeLoader.NativeFileName())));
            NativeLoader.EnsureInstalled();
            Assert.NotEqual(IntPtr.Zero, NativeLoader.LoadBundled(typeof(SzConfigTool).Assembly));
            Assert.NotEmpty(SzConfigTool.LibraryVersion);
        }

        [Fact]
        public void Rid_and_file_name_match_the_host()
        {
            string rid = NativeLoader.CurrentRid()!;
            Assert.Matches("^(linux|osx|win)-(x64|arm64)$", rid);
            string file = NativeLoader.NativeFileName();
            Assert.Contains("SzConfigTool", file);
        }

        // Host-independent: every OS / architecture mapping, not just this host's.
        [Theory]
        [InlineData("win", Architecture.X64, "win-x64")]
        [InlineData("osx", Architecture.Arm64, "osx-arm64")]
        [InlineData("linux", Architecture.X64, "linux-x64")]
        [InlineData("linux", Architecture.X86, null)]
        [InlineData(null, Architecture.Arm64, null)]
        public void Rid_maps_os_and_architecture(string? os, Architecture arch, string? rid)
        {
            Assert.Equal(rid, NativeLoader.Rid(os, arch));
        }

        [Theory]
        [InlineData("win", "SzConfigTool.dll")]
        [InlineData("osx", "libSzConfigTool.dylib")]
        [InlineData("linux", "libSzConfigTool.so")]
        [InlineData(null, "libSzConfigTool.so")]
        public void File_name_per_os(string? os, string file)
        {
            Assert.Equal(file, NativeLoader.FileName(os));
        }

        [Fact]
        public void Bundled_path_needs_a_supported_rid()
        {
            Assert.Null(NativeLoader.BundledPath("base", null, "libSzConfigTool.so"));
            Assert.Equal(Path.Combine("base", "runtimes", "linux-x64", "native", "libSzConfigTool.so"),
                NativeLoader.BundledPath("base", "linux-x64", "libSzConfigTool.so"));
        }

        [Fact]
        public void Host_os_is_one_of_the_known_ones()
        {
            Assert.Contains(NativeLoader.CurrentOs(), new[] { "win", "osx", "linux" });
        }

        // A runtime without NativeLibrary / DllImportResolver (.NET Framework),
        // or types without the expected members, leaves the default loader.
        [Fact]
        public void Binding_needs_native_library_and_resolver_types()
        {
            Assembly asm = typeof(SzConfigTool).Assembly;
            Type nativeLibrary = typeof(NativeLibrary);
            Assert.Null(NativeLoader.Bind(null, typeof(DllImportResolver), asm));
            Assert.Null(NativeLoader.Bind(nativeLibrary, null, asm));
            Assert.Null(NativeLoader.Bind(typeof(object), typeof(DllImportResolver), asm));
            Assert.Null(NativeLoader.Bind(nativeLibrary, typeof(Action), asm));
            Assert.NotNull(NativeLoader.Bind(nativeLibrary, typeof(DllImportResolver), asm));
            Assert.Equal(IntPtr.Zero, NativeLoader.LoadBundled(asm, null));
        }

        [Fact]
        public void Directory_of_an_assembly_location()
        {
            Assert.Equal(AppContext.BaseDirectory, NativeLoader.DirectoryOf(""));
            Assert.Equal(AppContext.BaseDirectory, NativeLoader.DirectoryOf(null));
            Assert.Equal(AppContext.BaseDirectory, NativeLoader.DirectoryOf("lib.dll"));
            Assert.Equal(AppContext.BaseDirectory, NativeLoader.DirectoryOf(Path.GetPathRoot(AssemblyDir)));
            Assert.Equal(AssemblyDir, NativeLoader.DirectoryOf(typeof(SzConfigTool).Assembly.Location));
        }

        [Fact]
        public void Installing_the_resolver_twice_is_harmless()
        {
            NativeLoader.EnsureInstalled();
            NativeLoader.EnsureInstalled();
            Assert.NotEmpty(SzConfigTool.LibraryVersion);
        }

        [Fact]
        public void Installing_over_an_existing_resolver_keeps_the_existing_one()
        {
            NativeLoader.EnsureInstalled();

            // A second registration for the same assembly is rejected by the
            // runtime (InvalidOperationException); the installed resolver wins.
            InvokePrivate("Install");
            Assert.NotEmpty(SzConfigTool.LibraryVersion);
        }

        [Fact]
        public void Resolver_only_answers_for_the_native_library()
        {
            NativeLoader.EnsureInstalled();
            Assembly asm = typeof(SzConfigTool).Assembly;
            Assert.Equal(IntPtr.Zero, (IntPtr)InvokePrivate("Resolve", "SomeOtherLibrary", asm, null)!);
            Assert.NotEqual(IntPtr.Zero, (IntPtr)InvokePrivate("Resolve", NativeMethods.LibraryName, asm, null)!);
        }

        [Fact]
        public void Assembly_without_a_location_loads_from_the_app_directory()
        {
            NativeLoader.EnsureInstalled();
            // Loaded from bytes, like a single-file app: Location is empty.
            Assembly inMemory = Assembly.Load(File.ReadAllBytes(typeof(Assert).Assembly.Location));
            Assert.Equal(string.Empty, inMemory.Location);
            Assert.True(File.Exists(NativeLoader.BundledPath(AppContext.BaseDirectory)));
            Assert.NotEqual(IntPtr.Zero, NativeLoader.LoadBundled(inMemory));
        }

        [Fact]
        public void Missing_bundled_library_yields_zero()
        {
            NativeLoader.EnsureInstalled();
            using var dir = new TempDir();
            Assert.Equal(IntPtr.Zero, NativeLoader.LoadBundled(CopyAssemblyInto(dir.Path)));
        }

        [Fact]
        public void Unloadable_bundled_library_yields_zero()
        {
            NativeLoader.EnsureInstalled();
            using var dir = new TempDir();
            string bundled = NativeLoader.BundledPath(dir.Path)!;
            Directory.CreateDirectory(Path.GetDirectoryName(bundled)!);
            File.WriteAllText(bundled, "not a shared library");
            Assert.Equal(IntPtr.Zero, NativeLoader.LoadBundled(CopyAssemblyInto(dir.Path)));
        }

        [Fact]
        public void Real_library_copy_next_to_an_assembly_is_loaded()
        {
            NativeLoader.EnsureInstalled();
            using var dir = new TempDir();
            string bundled = NativeLoader.BundledPath(dir.Path)!;
            Directory.CreateDirectory(Path.GetDirectoryName(bundled)!);
            File.Copy(NativeLoader.BundledPath(AssemblyDir)!, bundled);
            Assert.NotEqual(IntPtr.Zero, NativeLoader.LoadBundled(CopyAssemblyInto(dir.Path)));
        }

        private static object? InvokePrivate(string method, params object?[] args) =>
            typeof(NativeLoader).GetMethod(method, BindingFlags.NonPublic | BindingFlags.Static)!.Invoke(null, args);

        // A real assembly file (xunit.assert) loaded from inside dir, so its
        // Location is under dir.
        private static Assembly CopyAssemblyInto(string dir)
        {
            string source = typeof(Assert).Assembly.Location;
            string copy = Path.Combine(dir, Path.GetFileName(source));
            File.Copy(source, copy);
            return Assembly.LoadFile(copy);
        }

        private sealed class TempDir : IDisposable
        {
            public TempDir()
            {
                Path = System.IO.Path.Combine(System.IO.Path.GetTempPath(), "szct-" + Guid.NewGuid().ToString("N"));
                Directory.CreateDirectory(Path);
            }

            public string Path { get; }

            // Loaded assemblies keep their file open on Windows; best effort.
            public void Dispose()
            {
                try
                {
                    Directory.Delete(Path, true);
                }
                catch (UnauthorizedAccessException)
                {
                }
                catch (IOException)
                {
                }
            }
        }

        [Fact]
        public void Package_version_equals_native_library_version()
        {
            string info = typeof(SzConfigTool).Assembly.GetCustomAttribute<AssemblyInformationalVersionAttribute>()!.InformationalVersion;
            Assert.Equal(SzConfigTool.LibraryVersion, info.Split('+')[0]);
        }

        [Fact]
        public void Library_version_equals_workspace_version()
        {
            string cargo = File.ReadAllText(Path.Combine(Repo.Root, "Cargo.toml"));
            Match m = Regex.Match(cargo, @"\[workspace\.package\][^\[]*?\nversion = ""([^""]+)""");
            Assert.True(m.Success);
            Assert.Equal(m.Groups[1].Value, SzConfigTool.LibraryVersion);
            Assert.Equal(2, SzConfigTool.AbiVersion);
        }

        [Fact]
        public void Abi_version_matches_the_c_header()
        {
            string header = Path.Combine(Repo.Root, Repo.Manifest.GetProperty("paths").GetProperty("c_header").GetString()!);
            Match m = Regex.Match(File.ReadAllText(header), @"#define\s+SZCONFIGTOOL_ABI_VERSION\s+(\d+)");
            Assert.True(m.Success);
            Assert.Equal(int.Parse(m.Groups[1].Value), SzConfigTool.AbiVersion);
        }
    }
}
