using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using System.Threading;
using Sz.ConfigTool.Native;
using Sz.ConfigTool.Tests.Support;
using Xunit;

namespace Sz.ConfigTool.Tests
{
    /// <summary>Error mapping and per-thread isolation of the native last-error slot.</summary>
    public class ErrorTests
    {
        [Fact]
        public void Every_reason_code_has_a_kind_and_round_trips()
        {
            IReadOnlyList<string> codes = Repo.ReasonCodes;
            Assert.Equal(codes.Count + 1, Enum.GetValues(typeof(SzConfigToolErrorKind)).Length);
            foreach (string code in codes)
            {
                SzConfigToolErrorKind kind = SzConfigToolErrorKinds.FromReasonCode(code);
                Assert.NotEqual(SzConfigToolErrorKind.Unknown, kind);
                Assert.Equal(Repo.Pascal(code), kind.ToString());
                Assert.Equal(code, SzConfigToolErrorKinds.ToReasonCode(kind));
            }

            Assert.Equal(SzConfigToolErrorKind.Unknown, SzConfigToolErrorKinds.FromReasonCode(null));
            Assert.Equal(SzConfigToolErrorKind.Unknown, SzConfigToolErrorKinds.FromReasonCode("NOPE"));
            Assert.Null(SzConfigToolErrorKinds.ToReasonCode(SzConfigToolErrorKind.Unknown));
        }

        [Fact]
        public void Null_name_or_config_throws_before_the_native_call()
        {
            Assert.Equal("name", Assert.Throws<ArgumentNullException>(() => SzConfigTool.Invoke(null!, Repo.Fixture)).ParamName);
            Assert.Equal("configJson", Assert.Throws<ArgumentNullException>(() => SzConfigTool.Invoke("list_data_sources", null!)).ParamName);
        }

        [Fact]
        public void Null_args_are_sent_as_an_empty_object()
        {
            InvokeResult r = SzConfigTool.Invoke("list_data_sources", Repo.Fixture);
            Assert.Equal("json", r.Kind);
            Assert.Equal(SzConfigTool.ListDataSources(Repo.Fixture), r.Result);
        }

        public static TheoryData<string, Action<string>> KindMismatches => new()
        {
            // A real `json` function read through each other typed seam.
            { "config", config => NativeCall.Config("list_data_sources", config, "{}") },
            { "config_and_json", config => NativeCall.ConfigAndJson("list_data_sources", config, "{}") },
            { "int", config => NativeCall.Int("list_data_sources", config, "{}") },
            { "unit", config => NativeCall.Unit("list_data_sources", config, "{}") },
        };

        [Theory]
        [MemberData(nameof(KindMismatches))]
        public void Envelope_kind_mismatch_is_a_protocol_error(string expected, Action<string> call)
        {
            var e = Assert.Throws<SzConfigToolException>(() => call(Repo.Fixture));
            Assert.Equal(SzConfigToolErrorKind.Internal, e.Kind);
            Assert.Equal($"Sz.ConfigTool protocol error: list_data_sources: envelope kind 'json', expected '{expected}'", e.Message);
        }

        [Fact]
        public void Record_members_must_exist_in_a_json_object()
        {
            Assert.Equal(new[] { "1", "\"b\"" }, NativeCall.Members("{\"a\":1,\"b\":\"b\"}", "a", "b"));
            var missing = Assert.Throws<SzConfigToolException>(() => NativeCall.Members("{\"a\":1}", "a", "z"));
            Assert.Equal("Sz.ConfigTool protocol error: record has no field 'z'", missing.Message);
            var notObject = Assert.Throws<SzConfigToolException>(() => NativeCall.Members("[1]", "a"));
            Assert.Equal(SzConfigToolErrorKind.Internal, notObject.Kind);
            Assert.StartsWith("Sz.ConfigTool protocol error: record is not a JSON object: invalid JSON", notObject.Message);
        }

        [Fact]
        public void Every_reachable_reason_code_is_raised_by_the_real_library()
        {
            // Codes the conformance suite expects (each case asserts the exact
            // code); INTERNAL is not reachable from valid input.
            var expected = Repo.Conformance.GetProperty("cases").EnumerateArray()
                .SelectMany(c => c.GetProperty("steps").EnumerateArray())
                .Where(s => s.GetProperty("expect").TryGetProperty("error", out _))
                .Select(s => s.GetProperty("expect").GetProperty("error").GetString()!)
                .ToHashSet();
            Assert.Equal(new[] { "INTERNAL" }, Repo.ReasonCodes.Except(expected));
        }

        [Fact]
        public void Library_error_carries_kind_reason_and_message()
        {
            var e = Assert.Throws<SzConfigToolException>(() => SzConfigTool.GetDataSource(Repo.Fixture, "NOPE"));
            Assert.Equal(SzConfigToolErrorKind.NotFound, e.Kind);
            Assert.Equal("NOT_FOUND", e.ReasonCode);
            Assert.Contains("NOPE", e.Message);
            Assert.Null(e.Details);
            Assert.Equal(-2, e.ReturnCode);
        }

        [Fact]
        public void Reason_code_is_never_null()
        {
            // Like Java: a missing reason code is reported as INTERNAL (never null),
            // and Kind keeps the reason code's identity.
            var none = new SzConfigToolException(null, "m");
            Assert.Equal("INTERNAL", none.ReasonCode);
            Assert.Equal(SzConfigToolErrorKind.Internal, none.Kind);
            Assert.Equal(none.ReasonCode, SzConfigToolErrorKinds.ToReasonCode(none.Kind));
            string reason = new SzConfigToolException("NOT_FOUND", "m").ReasonCode;
            Assert.Equal("NOT_FOUND", reason);
            // An unrecognized code is kept verbatim; only Kind is Unknown.
            var future = new SzConfigToolException("SOME_FUTURE_CODE", "m");
            Assert.Equal("SOME_FUTURE_CODE", future.ReasonCode);
            Assert.Equal(SzConfigToolErrorKind.Unknown, future.Kind);
        }

        [Fact]
        public void Invalid_config_is_json_parse()
        {
            var e = Assert.Throws<SzConfigToolException>(() => SzConfigTool.ListDataSources("{not json"));
            Assert.Equal(SzConfigToolErrorKind.JsonParse, e.Kind);
        }

        [Fact]
        public void Validation_errors_carry_versioned_details()
        {
            var e = Assert.Throws<SzConfigToolException>(() =>
                SzConfigTool.AddSearchProfile(Repo.Fixture, "P2", "SEARCH", candidates: "Sometimes"));
            Assert.Equal(SzConfigToolErrorKind.ValidationErrors, e.Kind);
            JsonElement details = JsonDocument.Parse(e.Details!).RootElement;
            Assert.Equal("sz-configtool.validation-errors/v1", details.GetProperty("schema").GetString());
            Assert.True(details.GetProperty("failures").GetArrayLength() > 0);
        }

        [Theory]
        [InlineData("no_such_function", "{}")]
        [InlineData("list_data_sources", "[1]")]
        [InlineData("list_data_sources", "{\"unknown\":1}")]
        [InlineData("get_data_source", "{\"code\":1}")]
        [InlineData("get_data_source", "{\"code\":null}")]
        public void Wire_errors_are_invalid_input(string name, string args)
        {
            var e = Assert.Throws<SzConfigToolException>(() => SzConfigTool.Invoke(name, Repo.Fixture, args));
            Assert.Equal(SzConfigToolErrorKind.InvalidInput, e.Kind);
        }

        [Fact]
        public void Not_implemented_placeholders_stay_reachable_through_invoke()
        {
            var stubs = Repo.Functions.Where(f => !f.Implemented).Select(f => f.Name).ToHashSet();
            JsonElement step = Repo.Conformance.GetProperty("cases").EnumerateArray()
                .SelectMany(c => c.GetProperty("steps").EnumerateArray())
                .First(s => stubs.Contains(s.GetProperty("fn").GetString()!));
            var e = Assert.Throws<SzConfigToolException>(() => SzConfigTool.Invoke(
                step.GetProperty("fn").GetString()!, Repo.Fixture, step.GetProperty("args").GetRawText()));
            Assert.Equal(SzConfigToolErrorKind.NotImplemented, e.Kind);
        }

        [Fact]
        public void Errors_on_concurrent_threads_never_mix()
        {
            const int Iterations = 300;
            string fixture = Repo.Fixture;
            var failures = new List<string>();
            using var start = new Barrier(2);

            void NotFoundLoop()
            {
                start.SignalAndWait();
                for (int i = 0; i < Iterations; i++)
                {
                    var e = Assert.Throws<SzConfigToolException>(() => SzConfigTool.GetDataSource(fixture, "NOPE_" + i));
                    if (e.Kind != SzConfigToolErrorKind.NotFound || !e.Message.Contains("NOPE_" + i))
                    {
                        lock (failures) { failures.Add($"A{i}: {e.ReasonCode} {e.Message}"); }
                    }
                }
            }

            void MixedLoop()
            {
                start.SignalAndWait();
                for (int i = 0; i < Iterations; i++)
                {
                    SzConfigTool.ListDataSources(fixture);
                    var e = Assert.Throws<SzConfigToolException>(() => SzConfigTool.AddDataSource(fixture, "TEST"));
                    if (e.Kind != SzConfigToolErrorKind.AlreadyExists)
                    {
                        lock (failures) { failures.Add($"B{i}: {e.ReasonCode} {e.Message}"); }
                    }
                }
            }

            var a = new Thread(NotFoundLoop);
            var b = new Thread(MixedLoop);
            a.Start();
            b.Start();
            a.Join();
            b.Join();
            Assert.Empty(failures);
        }

        [Fact]
        public void Native_last_error_slot_is_thread_local()
        {
            Assert.Throws<SzConfigToolException>(() => SzConfigTool.GetDataSource(Repo.Fixture, "NOPE"));
            Assert.Equal("NOT_FOUND", Utf8.FromNative(NativeMethods.SzConfigTool_getLastErrorReasonCode()));

            string? otherThread = "unset";
            var t = new Thread(() => otherThread = Utf8.FromNative(NativeMethods.SzConfigTool_getLastErrorReasonCode()));
            t.Start();
            t.Join();
            Assert.Null(otherThread);
        }
    }
}
