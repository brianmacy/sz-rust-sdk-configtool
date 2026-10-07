using System;
using System.Linq;
using System.Text;
using System.Text.Json;
using Sz.ConfigTool.Native;
using Sz.ConfigTool.Tests.Support;
using Xunit;

namespace Sz.ConfigTool.Tests
{
    /// <summary>Configs are opaque: returned byte-exact, never re-serialized by the binding.</summary>
    public class OpacityTests
    {
        private static byte[] Z(string s) => Encoding.UTF8.GetBytes(s + "\0");

        /// <summary>The raw envelope text straight from the C ABI.</summary>
        private static string RawEnvelope(string name, string config, string args)
        {
            SzConfigToolResult r = NativeMethods.SzConfigTool_invoke(Z(name), Z(config), Z(args));
            Assert.Equal(0, r.ReturnCode);
            try
            {
                return Utf8.FromNative(r.Response)!;
            }
            finally
            {
                NativeMethods.SzConfigTool_free(r.Response);
            }
        }

        [Fact]
        public void Typed_config_equals_the_library_bytes()
        {
            const string Value = "\"é 😀 \\u0001 \\\" \\\\ / \\n tab\\t\"";
            string args = "{\"section_name\":\"CFG_DSRC\",\"field_name\":\"X_NOTE\",\"field_value\":" + Value + "}";
            // Decoded independently with System.Text.Json.
            string expected = JsonDocument.Parse(RawEnvelope("add_config_section_field", Repo.Fixture, args))
                .RootElement.GetProperty("config").GetString()!;

            ConfigAndJson typed = SzConfigTool.AddConfigSectionField(Repo.Fixture, "CFG_DSRC", "X_NOTE", Value);
            Assert.Equal(expected, typed.Config);
            Assert.Equal(Encoding.UTF8.GetBytes(expected), Encoding.UTF8.GetBytes(typed.Config));
            Assert.Equal(expected, SzConfigTool.Invoke("add_config_section_field", Repo.Fixture, args).Config);
        }

        [Fact]
        public void Unicode_and_escapes_round_trip_through_args_and_config()
        {
            const string Tricky = "é 😀 \u0001 \" \\ / \n \t \u2028 end";
            string raw = JsonSerializer.Serialize(Tricky);
            ConfigAndJson r = SzConfigTool.AddConfigSectionField(Repo.Fixture, "CFG_DSRC", "X_NOTE", raw);
            JsonElement rows = JsonDocument.Parse(SzConfigTool.GetConfigSection(r.Config, "CFG_DSRC")).RootElement;
            Assert.All(rows.EnumerateArray(), row => Assert.Equal(Tricky, row.GetProperty("X_NOTE").GetString()));

            string code = "dé_😀\"";
            string withDs = SzConfigTool.AddDataSource(Repo.Fixture, code);
            Assert.Contains(JsonDocument.Parse(SzConfigTool.ListDataSources(withDs)).RootElement.EnumerateArray(),
                d => d.GetProperty("dataSource").GetString() == code.ToUpperInvariant());
        }

        [Fact]
        public void Returned_config_is_accepted_verbatim_by_the_next_call()
        {
            string c1 = SzConfigTool.AddDataSource(Repo.Fixture, "A1");
            string c2 = SzConfigTool.AddDataSource(c1, "A2");
            string list = SzConfigTool.ListDataSources(c2);
            Assert.Contains("\"A1\"", list);
            Assert.Contains("\"A2\"", list);
        }

        [Fact]
        public void Configs_the_native_layer_cannot_carry_are_rejected()
        {
            Assert.Throws<ArgumentNullException>(() => SzConfigTool.ListDataSources(null!));
        }

        // A NUL character cannot cross the C ABI (NUL-terminated strings):
        // INVALID_INPUT like C++ (CONTRACT.md), not ArgumentException.
        [Fact]
        public void Nul_in_config_name_or_raw_args_is_invalid_input()
        {
            var typed = Assert.Throws<SzConfigToolException>(() => SzConfigTool.ListDataSources("{}\0"));
            Assert.Equal("INVALID_INPUT", typed.ReasonCode);
            Assert.Equal(SzConfigToolErrorKind.InvalidInput, typed.Kind);
            foreach (var (name, config, args) in new[]
            {
                ("list_data_sources", Repo.Fixture + "\0", "{}"),
                ("list_data_sources\0", Repo.Fixture, "{}"),
                ("add_data_source", Repo.Fixture, "{\"code\":\"A\0\"}"),
            })
            {
                var raw = Assert.Throws<SzConfigToolException>(() => SzConfigTool.Invoke(name, config, args));
                Assert.Equal("INVALID_INPUT", raw.ReasonCode);
            }
        }

        // In a typed string argument NUL is JSON-escaped (\u0000), so it
        // reaches the library unchanged.
        [Fact]
        public void Nul_in_a_typed_string_argument_reaches_the_library()
        {
            string config = SzConfigTool.AddDataSource(Repo.Fixture, "A\0B");
            Assert.Contains("A\\u0000B", SzConfigTool.ListDataSources(config));
        }

        // Built in code, not [InlineData]: attribute strings are stored as UTF-8
        // metadata, which turns a lone surrogate into U+FFFD.
        [Fact]
        public void Lone_surrogate_config_is_invalid_input()
        {
            foreach (string config in new[] { "{\"x\":\"\ud800\"}", "\udfff" })
            {
                // Same reason code as Java/TS/Python (CONTRACT.md), not ArgumentException.
                var e = Assert.Throws<SzConfigToolException>(() => SzConfigTool.ListDataSources(config));
                Assert.Equal("INVALID_INPUT", e.ReasonCode);
                Assert.Equal(SzConfigToolErrorKind.InvalidInput, e.Kind);
                var raw = Assert.Throws<SzConfigToolException>(() => SzConfigTool.Invoke("list_data_sources", config, "{}"));
                Assert.Equal("INVALID_INPUT", raw.ReasonCode);
            }
        }

        [Fact]
        public void Lone_surrogate_arg_is_invalid_input()
        {
            var e = Assert.Throws<SzConfigToolException>(() => SzConfigTool.AddDataSource(Repo.Fixture, "\ud800"));
            Assert.Equal("INVALID_INPUT", e.ReasonCode);
        }
    }
}
