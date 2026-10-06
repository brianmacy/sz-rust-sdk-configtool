using System;
using System.Linq;
using System.Reflection;
using System.Text.Json;
using Sz.ConfigTool.Tests.Support;
using Xunit;

namespace Sz.ConfigTool.Tests
{
    /// <summary>FieldUpdate, optional and required arguments, against the real library.</summary>
    public class ArgumentTests
    {
        private static JsonElement Row(string config, string section, string filter) =>
            JsonDocument.Parse(SzConfigTool.GetConfigSection(config, section, filter)).RootElement.EnumerateArray().Single();

        [Fact]
        public void FieldUpdate_default_is_leave()
        {
            FieldUpdate<string> d = default;
            Assert.True(d.IsLeave);
            Assert.Equal(FieldUpdate<string>.Leave, d);
            Assert.Equal(FieldUpdateState.Leave, d.State);
            Assert.Throws<InvalidOperationException>(() => d.Value);
        }

        [Fact]
        public void FieldUpdate_set_clear_and_implicit_conversion()
        {
            FieldUpdate<long> s = 7;
            Assert.True(s.IsSet);
            Assert.Equal(7, s.Value);
            Assert.Equal(FieldUpdate<long>.Set(7), s);
            Assert.NotEqual(FieldUpdate<long>.Set(8), s);
            Assert.True(FieldUpdate<long>.Clear.IsClear);
            Assert.NotEqual(FieldUpdate<long>.Clear, FieldUpdate<long>.Leave);
            Assert.Equal("Set(7)", s.ToString());
            Assert.Equal("Clear", FieldUpdate<long>.Clear.ToString());
        }

        [Fact]
        public void FieldUpdate_set_null_is_rejected()
        {
            Assert.Throws<ArgumentNullException>(() => FieldUpdate<string>.Set(null!));
        }

        [Fact]
        public void Tristate_leave_clear_set_reach_the_library()
        {
            string filter = "\"ERFRAG_CODE\": \"SNAME_SSTAB\"";
            string set = SzConfigTool.SetFragment(Repo.Fixture, "sname_sstab", description: "new desc");
            Assert.Equal("new desc", Row(set, "CFG_ERFRAG", filter).GetProperty("ERFRAG_DESC").GetString());

            string left = SzConfigTool.SetFragment(set, "sname_sstab");
            Assert.Equal("new desc", Row(left, "CFG_ERFRAG", filter).GetProperty("ERFRAG_DESC").GetString());

            string cleared = SzConfigTool.SetFragment(left, "sname_sstab", description: FieldUpdate<string>.Clear);
            Assert.Equal(JsonValueKind.Null, Row(cleared, "CFG_ERFRAG", filter).GetProperty("ERFRAG_DESC").ValueKind);
        }

        [Fact]
        public void Optional_arguments_are_omitted_when_null()
        {
            string config = SzConfigTool.AddDataSource(Repo.Fixture, "crm");
            JsonElement row = JsonDocument.Parse(SzConfigTool.GetDataSource(config, "CRM")).RootElement;
            Assert.Equal("Remember", row.GetProperty("RETENTION_LEVEL").GetString());

            string forget = SzConfigTool.AddDataSource(Repo.Fixture, "crm", retentionLevel: "forget", id: 4242);
            row = JsonDocument.Parse(SzConfigTool.GetDataSource(forget, "CRM")).RootElement;
            Assert.Equal("Forget", row.GetProperty("RETENTION_LEVEL").GetString());
            Assert.Equal(4242, row.GetProperty("DSRC_ID").GetInt64());
        }

        [Fact]
        public void Required_option_arguments_are_required_parameters()
        {
            foreach (ManifestFunction f in Repo.Functions.Where(f => f.Implemented && f.Args.Any(a => a.Required)))
            {
                foreach (MethodInfo m in TypedRunner.Methods(f))
                {
                    var ps = m.GetParameters().ToDictionary(p => p.Name!);
                    foreach (ManifestArg a in f.Args.Where(a => a.Required))
                    {
                        Assert.False(ps[Repo.Camel(a.Name)].IsOptional, $"{f.Name}.{a.Name}");
                    }
                }
            }
        }

        [Fact]
        public void Null_for_a_required_string_throws_before_the_native_call()
        {
            var e = Assert.Throws<ArgumentNullException>(() =>
                SzConfigTool.AddComparisonThreshold(Repo.Fixture, "STR_COMP", "all", null!));
            Assert.Equal("cfuncRtnval", e.ParamName);
        }

        [Fact]
        public void Required_argument_omitted_on_the_wire_is_missing_field()
        {
            var e = Assert.Throws<SzConfigToolException>(() => SzConfigTool.Invoke(
                "add_comparison_threshold", Repo.Fixture, "{\"cfunc_code\":\"STR_COMP\",\"ftype_code\":\"all\"}"));
            Assert.Equal(SzConfigToolErrorKind.MissingField, e.Kind);
        }

        [Fact]
        public void Raw_json_arguments_must_be_one_valid_value()
        {
            var e = Assert.Throws<ArgumentException>(() =>
                SzConfigTool.AddConfigSectionField(Repo.Fixture, "CFG_DSRC", "X", "1, \"injected\": 2"));
            Assert.Equal("fieldValue", e.ParamName);
            Assert.Throws<ArgumentException>(() => SzConfigTool.AddConfigSectionField(Repo.Fixture, "CFG_DSRC", "X", ""));
        }

        [Fact]
        public void String_list_arguments_reach_the_library()
        {
            ConfigAndJson r = SzConfigTool.AddComparisonCall(Repo.Fixture, "name_key", "exact_comp", new[] { "full_name", "given_name" });
            Assert.Equal(1000, JsonDocument.Parse(r.Json).RootElement.GetProperty("CFCALL_ID").GetInt64());
            JsonElement call = JsonDocument.Parse(SzConfigTool.ListComparisonCalls(r.Config)).RootElement
                .EnumerateArray().Single(c => c.GetProperty("id").GetInt64() == 1000);
            Assert.Equal(new[] { "FULL_NAME", "GIVEN_NAME" }, call.GetProperty("elementList").EnumerateArray().Select(e => e.GetString()));
            Assert.Throws<ArgumentException>(() => SzConfigTool.AddComparisonCall(Repo.Fixture, "name_key", "exact_comp", new[] { "full_name", null! }));
        }
    }
}
