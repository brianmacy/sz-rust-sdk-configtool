using System;
using System.IO;
using System.Linq;
using System.Reflection;
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
