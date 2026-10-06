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
