using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Sz.ConfigTool.Tests.Support;
using Xunit;

namespace Sz.ConfigTool.Tests
{
    /// <summary>
    /// Runs every case of api/manifest/generated/conformance.json through the
    /// generated typed API against the REAL native library and fixture.
    /// </summary>
    public class ConformanceTests
    {
        public static IEnumerable<object[]> Cases() =>
            Repo.Conformance.GetProperty("cases").EnumerateArray()
                .Select(c => new object[] { $"{c.GetProperty("group").GetString()}/{c.GetProperty("name").GetString()}" });

        [Theory]
        [MemberData(nameof(Cases))]
        public void Case_passes_against_real_library(string id)
        {
            JsonElement c = Repo.Conformance.GetProperty("cases").EnumerateArray()
                .Single(x => $"{x.GetProperty("group").GetString()}/{x.GetProperty("name").GetString()}" == id);
            string config = Repo.Fixture;
            int i = 0;
            foreach (JsonElement step in c.GetProperty("steps").EnumerateArray())
            {
                string? failure = RunStep(step, ref config);
                Assert.True(failure == null, $"{id} step {i}: {failure}");
                i++;
            }
        }

        [Fact]
        public void Every_manifest_function_is_exercised()
        {
            var exercised = Repo.Conformance.GetProperty("cases").EnumerateArray()
                .SelectMany(c => c.GetProperty("steps").EnumerateArray())
                .Select(s => s.GetProperty("fn").GetString())
                .ToHashSet();
            string[] missing = Repo.Functions.Select(f => f.Name).Where(n => !exercised.Contains(n)).ToArray();
            Assert.True(missing.Length == 0, "not exercised: " + string.Join(", ", missing));
        }

        [Theory]
        [InlineData("{\"fn\":\"get_data_source\",\"args\":{\"code\":\"TEST\"},\"expect\":{\"error\":\"NOT_FOUND\"}}")]
        [InlineData("{\"fn\":\"get_data_source\",\"args\":{\"code\":\"NOPE\"},\"expect\":{}}")]
        [InlineData("{\"fn\":\"get_data_source\",\"args\":{\"code\":\"TEST\"},\"expect\":{\"result\":{\"DSRC_ID\":-5}}}")]
        [InlineData("{\"fn\":\"list_data_sources\",\"args\":{},\"expect\":{\"len\":999}}")]
        [InlineData("{\"fn\":\"list_data_sources\",\"args\":{},\"expect\":{\"contains\":[{\"dsrcCode\":\"NOPE\"}]}}")]
        [InlineData("{\"fn\":\"list_data_sources\",\"args\":{},\"expect\":{\"kind\":\"config\"}}")]
        // An object result must never satisfy len/excludes vacuously.
        [InlineData("{\"fn\":\"get_data_source\",\"args\":{\"code\":\"TEST\"},\"expect\":{\"len\":0}}")]
        [InlineData("{\"fn\":\"get_data_source\",\"args\":{\"code\":\"TEST\"},\"expect\":{\"excludes\":[{\"DSRC_CODE\":\"NOPE\"}]}}")]
        public void Runner_reports_mismatches(string stepJson)
        {
            string config = Repo.Fixture;
            Assert.NotNull(RunStep(JsonDocument.Parse(stepJson).RootElement, ref config));
        }

        private static string? RunStep(JsonElement step, ref string config)
        {
            string fn = step.GetProperty("fn").GetString()!;
            string input = step.TryGetProperty("config_literal", out JsonElement lit) ? lit.GetString()! : config;
            bool wireOnly = step.TryGetProperty("wire_only", out JsonElement w) && w.GetBoolean();
            JsonElement expect = step.GetProperty("expect");
            StepOutcome o = TypedRunner.Run(fn, input, step.GetProperty("args"), wireOnly);

            if (expect.TryGetProperty("error", out JsonElement err))
            {
                return o.Error == null ? $"{fn}: succeeded but expected {err.GetString()}"
                    : o.Error.ReasonCode == err.GetString() && SzConfigToolErrorKinds.ToReasonCode(o.Error.Kind) == o.Error.ReasonCode ? null
                    : $"{fn}: error {o.Error.ReasonCode} ({o.Error.Message}) != {err.GetString()}";
            }

            if (o.Error != null)
            {
                return $"{fn}: failed {o.Error.ReasonCode} ({o.Error.Message})";
            }

            string? mismatch = CheckSuccess(expect, o);
            if (mismatch == null && o.Config != null)
            {
                config = o.Config;
            }

            return mismatch == null ? null : $"{fn}: {mismatch}";
        }

        private static string? CheckSuccess(JsonElement expect, StepOutcome o)
        {
            if (expect.TryGetProperty("kind", out JsonElement kind) && kind.GetString() != o.Kind)
            {
                return $"kind {o.Kind} != expected {kind.GetString()}";
            }

            using JsonDocument doc = JsonDocument.Parse(o.Result ?? "null");
            JsonElement result = doc.RootElement;
            if (expect.TryGetProperty("result", out JsonElement want) && !Subset.Matches(want, result))
            {
                return $"result {o.Result} does not contain {want.GetRawText()}";
            }

            bool inspectsArray = expect.TryGetProperty("len", out _) || expect.TryGetProperty("contains", out _)
                || expect.TryGetProperty("excludes", out _);
            if (!inspectsArray)
            {
                return null;
            }

            if (result.ValueKind != JsonValueKind.Array)
            {
                return $"len/contains/excludes need an array result, got {o.Result}";
            }

            List<JsonElement> items = result.EnumerateArray().ToList();
            if (expect.TryGetProperty("len", out JsonElement len) && items.Count != len.GetInt32())
            {
                return $"len {items.Count} != expected {len.GetInt32()}";
            }

            if (expect.TryGetProperty("contains", out JsonElement contains))
            {
                foreach (JsonElement c in contains.EnumerateArray().Where(c => !items.Any(it => Subset.Matches(c, it))))
                {
                    return $"no element matches {c.GetRawText()}";
                }
            }

            if (expect.TryGetProperty("excludes", out JsonElement excludes))
            {
                foreach (JsonElement x in excludes.EnumerateArray().Where(x => items.Any(it => Subset.Matches(x, it))))
                {
                    return $"an element matches excluded {x.GetRawText()}";
                }
            }

            return null;
        }
    }
}
