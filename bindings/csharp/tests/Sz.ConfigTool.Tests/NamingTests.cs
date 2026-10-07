using System;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;
using Sz.ConfigTool.Tests.Support;
using Xunit;

namespace Sz.ConfigTool.Tests
{
    /// <summary>The generated API matches the manifest and the naming contract.</summary>
    public class NamingTests
    {
        private static readonly HashSet<string> HandWritten = new() { nameof(SzConfigTool.Invoke) };

        [Fact]
        public void Every_implemented_function_has_a_pascal_case_method()
        {
            foreach (ManifestFunction f in Repo.Functions.Where(f => f.Implemented))
            {
                foreach (MethodInfo m in TypedRunner.Methods(f))
                {
                    ParameterInfo[] ps = m.GetParameters();
                    Assert.Equal("configJson", ps[0].Name);
                    Assert.Equal(typeof(string), ps[0].ParameterType);
                    Assert.Equal(f.Args.Select(a => Repo.Camel(a.Name)).OrderBy(n => n), ps.Skip(1).Select(p => p.Name).OrderBy(n => n));
                }
            }
        }

        [Fact]
        public void Every_implemented_function_and_nothing_else_is_generated()
        {
            // Plus the <Name>Result companion of every config_and_json function.
            var expected = Repo.Functions.Where(f => f.Implemented)
                .SelectMany(f => f.Returns == "config_and_json"
                    ? new[] { Repo.Pascal(f.Name), Repo.Pascal(f.Name) + "Result" }
                    : new[] { Repo.Pascal(f.Name) })
                .ToHashSet();
            var actual = typeof(SzConfigTool).GetMethods(BindingFlags.Public | BindingFlags.Static)
                .Where(m => !m.IsSpecialName && !HandWritten.Contains(m.Name))
                .Select(m => m.Name)
                .ToHashSet();
            Assert.Equal(expected.OrderBy(n => n), actual.OrderBy(n => n));
            foreach (ManifestFunction f in Repo.Functions.Where(f => !f.Implemented))
            {
                Assert.Null(typeof(SzConfigTool).GetMethod(Repo.Pascal(f.Name)));
            }
        }

        [Fact]
        public void Parameter_shapes_follow_required_optional_and_tristate()
        {
            foreach (ManifestFunction f in Repo.Functions.Where(f => f.Implemented))
            foreach (MethodInfo m in TypedRunner.Methods(f))
            {
                Dictionary<string, ParameterInfo> ps = m.GetParameters().Skip(1).ToDictionary(p => p.Name!);
                foreach (ManifestArg a in f.Args)
                {
                    ParameterInfo p = ps[Repo.Camel(a.Name)];
                    string at = $"{f.Name}.{a.Name}";
                    if (a.Tristate)
                    {
                        Assert.True(p.ParameterType.IsGenericType && p.ParameterType.GetGenericTypeDefinition() == typeof(FieldUpdate<>), at);
                        Assert.True(p.HasDefaultValue, at);
                    }
                    else if (a.Optional && !a.Required)
                    {
                        Assert.True(p.HasDefaultValue && p.DefaultValue == null, at);
                    }
                    else
                    {
                        Assert.False(p.HasDefaultValue, at);
                        Assert.Null(Nullable.GetUnderlyingType(p.ParameterType));
                    }
                }
            }
        }

        [Fact]
        public void Return_types_follow_manifest_returns()
        {
            foreach (ManifestFunction f in Repo.Functions.Where(f => f.Implemented && f.TupleNames.Count == 0))
            {
                Type rt = TypedRunner.Methods(f).Select(m => m.ReturnType).Distinct().Single();
                Type expected = f.Returns switch
                {
                    "config" or "json" or "config_and_json" => typeof(string),
                    "int" => typeof(long),
                    "unit" => typeof(void),
                    _ => throw new InvalidOperationException(f.Returns),
                };
                Assert.True(expected.IsAssignableFrom(rt) || (expected == typeof(void) && rt == typeof(void)), f.Name);
            }
        }

        [Fact]
        public void Casing_splits_on_underscore_without_acronyms()
        {
            Assert.NotNull(typeof(SzConfigTool).GetMethod("AddDataSource"));
            Assert.Equal("GetFtypeId", Repo.Pascal("get_ftype_id"));
            ParameterInfo cls = typeof(SzConfigTool).GetMethod("AddAttribute")!.GetParameters().Single(p => p.Name == "class");
            Assert.Equal(typeof(string), cls.ParameterType);
        }

        [Fact]
        public void Int_or_str_args_get_long_and_string_overloads()
        {
            ManifestFunction[] fs = Repo.Functions.Where(f => f.Implemented && f.Args.Any(a => a.Type == "int_or_str")).ToArray();
            Assert.NotEmpty(fs);
            foreach (ManifestFunction f in fs)
            {
                string[] sel = f.Args.Where(a => a.Type == "int_or_str").Select(a => Repo.Camel(a.Name)).ToArray();
                IReadOnlyList<MethodInfo> ms = TypedRunner.Methods(f);
                Assert.Equal(1 << sel.Length, ms.Count);
                foreach (string p in sel)
                {
                    var types = ms.Select(m => m.GetParameters().Single(x => x.Name == p).ParameterType).ToHashSet();
                    Assert.True(types.SetEquals(new[] { typeof(long), typeof(string) }), $"{f.Name}.{p}");
                }
            }
        }

        [Fact]
        public void Tuple_names_return_a_named_record_of_json_text_fields()
        {
            ManifestFunction[] fs = Repo.Functions.Where(f => f.Implemented && f.TupleNames.Count > 0).ToArray();
            Assert.Contains(fs, f => f.Returns == "config_and_json");
            Assert.Contains(fs, f => f.Returns == "json");
            foreach (ManifestFunction f in fs)
            {
                MethodInfo m = TypedRunner.Methods(f).Single();
                Type rt = f.Returns == "config_and_json" ? TypedRunner.Companion(f, m).ReturnType : m.ReturnType;
                Assert.Equal(Repo.Pascal(f.Name) + "Record", rt.Name);
                Assert.Null(rt.GetProperty("Json"));
                Assert.Null(rt.GetProperty("Config"));
                var expected = f.TupleNames.Select(Repo.Pascal).ToList();

                Assert.Equal(expected, rt.GetProperties().Where(p => p.Name != "EqualityContract").Select(p => p.Name).OrderBy(n => expected.IndexOf(n)).ToList());
                Assert.All(rt.GetProperties().Where(p => p.Name != "EqualityContract"), p => Assert.Equal(typeof(string), p.PropertyType));
            }
        }
    }
}
