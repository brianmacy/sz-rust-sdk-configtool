using System;
using System.Linq;
using System.Reflection;
using System.Text.Json;
using Sz.ConfigTool.Tests.Support;
using Xunit;

namespace Sz.ConfigTool.Tests
{
    /// <summary>
    /// Every config-changing method returns the new config text (issue #75): a
    /// <c>config_and_json</c> function's primary returns only the config; its
    /// companion <c>&lt;Name&gt;Result</c> (same parameters) returns the record
    /// JSON text, or the named-fields record for <c>tuple_names</c>.
    /// </summary>
    public class ChainingTests
    {
        private static MethodInfo[] Named(string name) =>
            typeof(SzConfigTool).GetMethods(BindingFlags.Public | BindingFlags.Static).Where(m => m.Name == name).ToArray();

        [Fact]
        public void Config_changing_primaries_return_string_and_pairs_have_companions()
        {
            int paired = 0;
            foreach (ManifestFunction f in Repo.Functions.Where(f => f.Implemented))
            {
                string name = Repo.Pascal(f.Name);
                MethodInfo[] companions = Named(name + "Result");
                if (f.Returns != "config_and_json")
                {
                    Assert.Empty(companions);
                }

                if (f.Returns != "config" && f.Returns != "config_and_json")
                {
                    continue;
                }

                MethodInfo[] primaries = Named(name);
                Assert.NotEmpty(primaries);
                Assert.All(primaries, m => Assert.Equal(typeof(string), m.ReturnType));
                if (f.Returns != "config_and_json")
                {
                    continue;
                }

                paired++;
                Assert.Equal(primaries.Length, companions.Length);
                string want = f.TupleNames.Count > 0 ? name + "Record" : "String";
                foreach (MethodInfo p in primaries)
                {
                    Type[] types = p.GetParameters().Select(x => x.ParameterType).ToArray();
                    MethodInfo? c = typeof(SzConfigTool).GetMethod(name + "Result", types);
                    Assert.True(c != null, $"{name}Result({string.Join(", ", types.Select(t => t.Name))})");
                    Assert.Equal(want, c!.ReturnType.Name);
                    Assert.Equal(p.GetParameters().Select(x => (x.Name, x.HasDefaultValue)),
                        c.GetParameters().Select(x => (x.Name, x.HasDefaultValue)));
                }
            }

            Assert.True(paired > 0);
        }

        [Fact]
        public void ConfigAndJson_type_is_gone()
        {
            Assert.Null(typeof(SzConfigTool).Assembly.GetType("Sz.ConfigTool.ConfigAndJson"));
        }

        [Fact]
        public void Chaining_seven_config_changing_functions()
        {
            string cfg = Repo.Fixture;
            cfg = SzConfigTool.AddElement(cfg, "DEMO_EL", dataType: "string");
            cfg = SzConfigTool.AddFeature(cfg, "DEMO_FEAT", "[\"DEMO_EL\"]");
            cfg = SzConfigTool.AddAttribute(cfg, "DEMO_ATTR", "DEMO_FEAT", "DEMO_EL", "OTHER");
            cfg = SzConfigTool.AddFragment(cfg, "{\"ERFRAG_CODE\":\"DEMO_FRAG\",\"ERFRAG_SOURCE\":\"./FRAGMENT[./SAME_NAME>0]\"}");
            cfg = SzConfigTool.AddComparisonCall(cfg, "DEMO_FEAT", "EXACT_COMP", new[] { "DEMO_EL" });
            cfg = SzConfigTool.AddComparisonFunction(cfg, "DEMO_COMP");
            cfg = SzConfigTool.AddDataSource(cfg, "DEMO_DS");
            Assert.Equal("DEMO_ATTR", JsonDocument.Parse(SzConfigTool.GetAttribute(cfg, "DEMO_ATTR")).RootElement.GetProperty("ATTR_CODE").GetString());
            Assert.Contains("DEMO_FRAG", SzConfigTool.GetFragment(cfg, "DEMO_FRAG"));
            Assert.Contains("DEMO_COMP", SzConfigTool.GetComparisonFunction(cfg, "DEMO_COMP"));
            Assert.Contains("DEMO_DS", SzConfigTool.ListDataSources(cfg));
        }

        [Fact]
        public void Companion_returns_the_row_of_the_same_operation()
        {
            string row = SzConfigTool.AddAttributeResult(Repo.Fixture, "X_ATTR", "NAME", "FULL_NAME", "OTHER");
            InvokeResult wire = SzConfigTool.Invoke("add_attribute", Repo.Fixture,
                "{\"attribute\":\"X_ATTR\",\"feature\":\"NAME\",\"element\":\"FULL_NAME\",\"class\":\"OTHER\"}");
            Assert.Equal(wire.Result, row);
            Assert.Equal(wire.Config, SzConfigTool.AddAttribute(Repo.Fixture, "X_ATTR", "NAME", "FULL_NAME", "OTHER"));
        }

        [Fact]
        public void Tuple_names_companion_is_the_named_record_only()
        {
            SetGenericPlanRecord created = SzConfigTool.SetGenericPlanResult(Repo.Fixture, "new_plan", "New Plan");
            Assert.Equal(new SetGenericPlanRecord("3", "true"), created);
            string cfg = SzConfigTool.SetGenericPlan(Repo.Fixture, "new_plan", "New Plan");
            Assert.Contains("NEW_PLAN", cfg);
            SetGenericPlanRecord updated = SzConfigTool.SetGenericPlanResult(cfg, "NEW_PLAN", "Renamed");
            Assert.Equal(new SetGenericPlanRecord("3", "false"), updated);
        }
    }
}
