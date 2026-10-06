using System.Linq;
using System.Text.Json;

namespace Sz.ConfigTool.Tests.Support
{
    /// <summary>The conformance subset-match rules (api/manifest/schema.md).</summary>
    public static class Subset
    {
        /// <summary>Objects by key (recursively), arrays element-wise with equal length, scalars by equality.</summary>
        public static bool Matches(JsonElement expected, JsonElement actual)
        {
            switch (expected.ValueKind)
            {
                case JsonValueKind.Object:
                    return actual.ValueKind == JsonValueKind.Object && expected.EnumerateObject().All(p =>
                        actual.TryGetProperty(p.Name, out JsonElement a) && Matches(p.Value, a));
                case JsonValueKind.Array:
                    return actual.ValueKind == JsonValueKind.Array
                        && expected.GetArrayLength() == actual.GetArrayLength()
                        && expected.EnumerateArray().Zip(actual.EnumerateArray()).All(x => Matches(x.First, x.Second));
                case JsonValueKind.Number:
                    return actual.ValueKind == JsonValueKind.Number && NumbersEqual(expected, actual);
                default:
                    return expected.ValueKind == actual.ValueKind
                        && (expected.ValueKind != JsonValueKind.String || expected.GetString() == actual.GetString());
            }
        }

        private static bool NumbersEqual(JsonElement e, JsonElement a)
        {
            if (e.TryGetInt64(out long el) && a.TryGetInt64(out long al))
            {
                return el == al;
            }

            return e.GetDouble() == a.GetDouble();
        }
    }
}
