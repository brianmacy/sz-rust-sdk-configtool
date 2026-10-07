using System.Linq;
using System.Text.Json;
using Sz.ConfigTool.Tests.Support;
using Xunit;

namespace Sz.ConfigTool.Tests
{
    /// <summary>Named result records and int_or_str overloads against the REAL library and fixture.</summary>
    public class TypedShapeTests
    {
        [Fact]
        public void Set_generic_plan_returns_config_plan_id_and_was_created_as_json_text()
        {
            SetGenericPlanResult created = SzConfigTool.SetGenericPlan(Repo.Fixture, "new_plan", "New Plan");
            Assert.Equal("3", created.PlanId);
            Assert.Equal("true", created.WasCreated);
            Assert.Contains("NEW_PLAN", created.Config);

            SetGenericPlanResult updated = SzConfigTool.SetGenericPlan(Repo.Fixture, "search", "Updated");
            Assert.Equal("2", updated.PlanId);
            Assert.Equal("false", updated.WasCreated);
        }

        [Fact]
        public void Verify_compatibility_version_returns_named_json_text_fields()
        {
            VerifyCompatibilityVersionResult match = SzConfigTool.VerifyCompatibilityVersion(Repo.Fixture, "11");
            Assert.Equal("\"11\"", match.CurrentVersion);
            Assert.Equal("true", match.Matches);

            VerifyCompatibilityVersionResult mismatch = SzConfigTool.VerifyCompatibilityVersion(Repo.Fixture, "11.0");
            Assert.Equal("\"11\"", mismatch.CurrentVersion);
            Assert.Equal("false", mismatch.Matches);
        }

        [Fact]
        public void Call_selector_by_id_and_by_feature_code_agree()
        {
            string byId = SzConfigTool.GetComparisonCall(Repo.Fixture, 34L);
            string byCode = SzConfigTool.GetComparisonCall(Repo.Fixture, "tax_id");
            Assert.Equal(byId, byCode);
            Assert.Equal(34, JsonDocument.Parse(byId).RootElement.GetProperty("CFCALL_ID").GetInt64());
        }

        [Fact]
        public void Call_selector_type_reaches_the_wire()
        {
            // "4" is a feature code, not the id 4: unknown feature = NOT_FOUND.
            var e = Assert.Throws<SzConfigToolException>(() => SzConfigTool.GetComparisonCall(Repo.Fixture, "4"));
            Assert.Equal(SzConfigToolErrorKind.NotFound, e.Kind);
            Assert.Equal(4, JsonDocument.Parse(SzConfigTool.GetComparisonCall(Repo.Fixture, 4L)).RootElement.GetProperty("CFCALL_ID").GetInt64());
        }

        [Fact]
        public void Delete_call_element_by_feature_code_and_by_id()
        {
            // Comparison call 4 (GENDER) gets a second element, FULL_NAME.
            string config = SzConfigTool.AddComparisonCallElement(Repo.Fixture, 4, 4, 2).Config;
            string byCode = SzConfigTool.DeleteComparisonCallElement(config, "gender", "full_name");
            Assert.Equal(new[] { "GENDER" }, Elements(byCode, 4));
            string byId = SzConfigTool.DeleteComparisonCallElement(byCode, 4L, "GENDER");
            Assert.Empty(Elements(byId, 4));
            var e = Assert.Throws<SzConfigToolException>(() => SzConfigTool.DeleteComparisonCallElement(Repo.Fixture, 9999L, "GENDER"));
            Assert.Equal(SzConfigToolErrorKind.NotFound, e.Kind);
        }

        private static string?[] Elements(string config, long callId) =>
            JsonDocument.Parse(SzConfigTool.ListComparisonCalls(config)).RootElement.EnumerateArray()
                .Single(c => c.GetProperty("id").GetInt64() == callId)
                .GetProperty("elementList").EnumerateArray().Select(x => x.GetString()).ToArray();
    }
}
