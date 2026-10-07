using System;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;
using System.Text.Json;
using Xunit;

namespace Sz.ConfigTool.Tests.Support
{
    /// <summary>The outcome of one step: kind + config + result JSON, or an error.</summary>
    public sealed record StepOutcome(string? Kind, string? Config, string? Result, SzConfigToolException? Error);

    /// <summary>
    /// Executes a conformance step through the GENERATED typed method (found
    /// by its contract name), converting the step's JSON args to the typed
    /// parameters. Steps the typed API cannot express (wire_only, or a
    /// not_implemented function) go through <see cref="SzConfigTool.Invoke"/>.
    /// </summary>
    public static class TypedRunner
    {
        public static StepOutcome Run(string fn, string config, JsonElement args, bool wireOnly)
        {
            ManifestFunction f = Repo.Function(fn);
            return wireOnly || !f.Implemented ? RunWire(fn, config, args) : RunTyped(f, config, args);
        }

        /// <summary>Every typed overload of <paramref name="f"/> (2^n for n <c>int_or_str</c> args).</summary>
        public static IReadOnlyList<MethodInfo> Methods(ManifestFunction f)
        {
            MethodInfo[] ms = typeof(SzConfigTool).GetMethods(BindingFlags.Public | BindingFlags.Static)
                .Where(m => m.Name == Repo.Pascal(f.Name)).ToArray();
            return ms.Length > 0 ? ms : throw new InvalidOperationException($"no typed method {Repo.Pascal(f.Name)} for {f.Name}");
        }

        /// <summary>
        /// The overload matching the JSON type of each <c>int_or_str</c> arg in
        /// <paramref name="args"/> (number = <c>long</c>, string = <c>string</c>).
        /// </summary>
        public static MethodInfo Method(ManifestFunction f, JsonElement args) =>
            Methods(f).Single(m => f.Args.Where(a => a.Type == "int_or_str").All(a =>
            {
                Type t = m.GetParameters().Single(p => p.Name == Repo.Camel(a.Name)).ParameterType;
                return !args.TryGetProperty(a.Name, out JsonElement v)
                    || t == (v.ValueKind == JsonValueKind.Number ? typeof(long) : typeof(string));
            }));

        private static StepOutcome RunWire(string fn, string config, JsonElement args)
        {
            try
            {
                InvokeResult r = SzConfigTool.Invoke(fn, config, args.GetRawText());
                return new StepOutcome(r.Kind, r.Config, r.Result, null);
            }
            catch (SzConfigToolException e)
            {
                return new StepOutcome(null, null, null, e);
            }
        }

        private static StepOutcome RunTyped(ManifestFunction f, string config, JsonElement args)
        {
            MethodInfo m = Method(f, args);
            Dictionary<string, ManifestArg> byParam = f.Args.ToDictionary(a => Repo.Camel(a.Name));
            var known = new HashSet<string>(args.EnumerateObject().Select(p => p.Name));
            object?[] values = m.GetParameters().Select((p, i) =>
                i == 0 ? config : ParamValue(p, byParam[p.Name!], args, known)).ToArray();
            if (known.Count > 0)
            {
                throw new InvalidOperationException($"{f.Name}: args without a typed parameter: {string.Join(", ", known)}");
            }

            object? ret;
            try
            {
                ret = m.Invoke(null, values);
            }
            catch (TargetInvocationException e) when (e.InnerException is SzConfigToolException sz)
            {
                if (f.Returns == "config_and_json")
                {
                    // The companion must fail the same way.
                    var other = Assert.Throws<TargetInvocationException>(() => Companion(f, m).Invoke(null, values));
                    Assert.Equal(sz.ReasonCode, Assert.IsType<SzConfigToolException>(other.InnerException).ReasonCode);
                }

                return new StepOutcome(null, null, null, sz);
            }

            if (f.Returns == "config_and_json")
            {
                // Primary = the config text; companion (same args) = the record.
                string config2 = Assert.IsType<string>(ret);
                object rec = Companion(f, m).Invoke(null, values)!;
                string result = f.TupleNames.Count > 0 ? NamedRecord(f, rec).Result! : Assert.IsType<string>(rec);
                return new StepOutcome(f.Returns, config2, result, null);
            }

            return ret switch
            {
                _ when f.TupleNames.Count > 0 => NamedRecord(f, ret!),
                string s when f.Returns == "config" => new StepOutcome(f.Returns, s, null, null),
                string s => new StepOutcome(f.Returns, null, s, null),
                long l => new StepOutcome(f.Returns, null, l.ToString(), null),
                _ => new StepOutcome(f.Returns, null, null, null),
            };
        }

        /// <summary>The <c>&lt;Name&gt;Result</c> companion overload matching primary <paramref name="m"/>.</summary>
        public static MethodInfo Companion(ManifestFunction f, MethodInfo m) =>
            typeof(SzConfigTool).GetMethod(Repo.Pascal(f.Name) + "Result", m.GetParameters().Select(p => p.ParameterType).ToArray())
            ?? throw new InvalidOperationException($"no companion {Repo.Pascal(f.Name)}Result for {f.Name}");

        // A <Fn>Record: rebuild {"name": <field JSON text>, ...} from its
        // PascalCase properties, so the conformance expectation applies as-is.
        private static StepOutcome NamedRecord(ManifestFunction f, object rec)
        {
            Type t = rec.GetType();
            Assert.Equal(f.TupleNames.Count, t.GetProperties().Count(p => p.Name != "EqualityContract"));
            string json = "{" + string.Join(",", f.TupleNames.Select(n =>
                $"\"{n}\":{(string)t.GetProperty(Repo.Pascal(n))!.GetValue(rec)!}")) + "}";
            return new StepOutcome(f.Returns, null, json, null);
        }

        private static object? ParamValue(ParameterInfo p, ManifestArg a, JsonElement args, HashSet<string> known)
        {
            if (!args.TryGetProperty(a.Name, out JsonElement v))
            {
                return p.HasDefaultValue ? Missing(p) : throw new InvalidOperationException($"required arg {a.Name} absent");
            }

            known.Remove(a.Name);
            if (a.Tristate)
            {
                return TriState(p.ParameterType, a, v);
            }

            return Convert(a, v);
        }

        // An omitted optional parameter: null, or default(FieldUpdate<T>) = Leave.
        private static object? Missing(ParameterInfo p) =>
            p.ParameterType.IsValueType && Nullable.GetUnderlyingType(p.ParameterType) == null
                ? Activator.CreateInstance(p.ParameterType)
                : null;

        private static object TriState(Type fieldUpdate, ManifestArg a, JsonElement v)
        {
            if (v.ValueKind == JsonValueKind.Null)
            {
                return fieldUpdate.GetProperty("Clear")!.GetValue(null)!;
            }

            return fieldUpdate.GetMethod("Set")!.Invoke(null, new[] { Convert(a, v) })!;
        }

        private static object Convert(ManifestArg a, JsonElement v) => a.Type switch
        {
            "str" => v.GetString()!,
            "int" => v.GetInt64(),
            "bool" => v.GetBoolean(),
            "json" => v.GetRawText(),
            "int_or_str" => v.ValueKind == JsonValueKind.Number ? v.GetInt64() : v.GetString()!,
            "str_list" => v.EnumerateArray().Select(e => e.GetString()!).ToList(),
            _ => throw new InvalidOperationException($"unknown arg type {a.Type}"),
        };
    }
}
