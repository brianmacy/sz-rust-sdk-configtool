using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text.Json;

namespace Sz.ConfigTool.Tests.Support
{
    /// <summary>One manifest argument (subset of manifest.json fields the tests need).</summary>
    public sealed record ManifestArg(string Name, string Type, bool Optional, bool Tristate, bool Required);

    /// <summary>One manifest function.</summary>
    public sealed record ManifestFunction(string Name, string Returns, string Status, IReadOnlyList<ManifestArg> Args, IReadOnlyList<string> TupleNames)
    {
        public bool Implemented => Status != "not_implemented";
    }

    /// <summary>Locates and loads the generated manifest, conformance cases and the real fixture.</summary>
    public static class Repo
    {
        private static readonly Lazy<JsonDocument> ManifestDoc = new(() => Parse(Path.Combine(Root, Metadata("SzManifestJson"))));
        private static readonly Lazy<JsonDocument> ConformanceDoc = new(() => Parse(Path.Combine(Root, ManifestPath("conformance_json_out"))));

        public static string Root => Metadata("SzRepoRoot");

        public static JsonElement Manifest => ManifestDoc.Value.RootElement;

        public static JsonElement Conformance => ConformanceDoc.Value.RootElement;

        /// <summary>The REAL template config every conformance case starts from.</summary>
        public static string Fixture => File.ReadAllText(Path.Combine(Root, Conformance.GetProperty("fixture").GetString()!));

        public static IReadOnlyList<string> ReasonCodes =>
            Manifest.GetProperty("reason_codes").EnumerateArray().Select(e => e.GetString()!).ToList();

        public static IReadOnlyList<ManifestFunction> Functions { get; } = LoadFunctions();

        public static ManifestFunction Function(string name) => Functions.Single(f => f.Name == name);

        /// <summary>Contract casing: split on '_', capitalize each part, no acronym handling.</summary>
        public static string Pascal(string snake) =>
            string.Concat(snake.Split('_').Where(p => p.Length > 0).Select(p => char.ToUpperInvariant(p[0]) + p.Substring(1).ToLowerInvariant()));

        public static string Camel(string snake)
        {
            string p = Pascal(snake);
            return p.Length == 0 ? p : char.ToLowerInvariant(p[0]) + p.Substring(1);
        }

        private static string ManifestPath(string key) => Manifest.GetProperty("paths").GetProperty(key).GetString()!;

        private static JsonDocument Parse(string path) => JsonDocument.Parse(File.ReadAllText(path));

        private static string Metadata(string key) =>
            typeof(Repo).Assembly.GetCustomAttributes<AssemblyMetadataAttribute>().Single(a => a.Key == key).Value!;

        private static IReadOnlyList<ManifestFunction> LoadFunctions() =>
            Manifest.GetProperty("functions").EnumerateArray().Select(f => new ManifestFunction(
                f.GetProperty("name").GetString()!,
                f.GetProperty("returns").GetString()!,
                f.GetProperty("status").GetString()!,
                f.GetProperty("args").EnumerateArray().Select(a => new ManifestArg(
                    a.GetProperty("name").GetString()!,
                    a.GetProperty("type").GetString()!,
                    a.GetProperty("optional").GetBoolean(),
                    a.GetProperty("tristate").GetBoolean(),
                    a.GetProperty("required").GetBoolean())).ToList(),
                f.TryGetProperty("tuple_names", out JsonElement t)
                    ? t.EnumerateArray().Select(e => e.GetString()!).ToList()
                    : new List<string>())).ToList();
    }
}
