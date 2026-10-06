using System;
using System.Collections.Generic;
using System.Text.Json;
using Sz.ConfigTool.Json;
using Xunit;

namespace Sz.ConfigTool.Tests
{
    /// <summary>The dependency-free args writer and envelope scanner.</summary>
    public class JsonTests
    {
        [Theory]
        [InlineData("1")]
        [InlineData("-0.5e+10")]
        [InlineData(" {\"a\": [1, true, null, \"x\\u00e9\"]} ")]
        [InlineData("\"\\ud83d\\ude00\"")]
        [InlineData("[]")]
        public void Valid_single_values_are_accepted(string json)
        {
            Assert.True(JsonScanner.IsSingleValue(json));
        }

        [Theory]
        [InlineData("")]
        [InlineData("1 2")]
        [InlineData("1, \"x\": 2")]
        [InlineData("{\"a\":1,}")]
        [InlineData("[1,]")]
        [InlineData("01")]
        [InlineData("\"a\nb\"")]
        [InlineData("\"\\x\"")]
        [InlineData("tru")]
        [InlineData("{'a':1}")]
        public void Invalid_json_is_rejected(string json)
        {
            Assert.False(JsonScanner.IsSingleValue(json));
        }

        [Fact]
        public void Object_members_are_raw_spans()
        {
            IReadOnlyDictionary<string, string> m = JsonScanner.ObjectMembers("{\"kind\":\"json\",\"result\": {\"a\" : [1, 2]} }");
            Assert.Equal("\"json\"", m["kind"]);
            Assert.Equal("{\"a\" : [1, 2]}", m["result"]);
            Assert.Empty(JsonScanner.ObjectMembers(" {} "));
            Assert.Throws<FormatException>(() => JsonScanner.ObjectMembers("{\"a\":1,\"a\":2}"));
            Assert.Throws<FormatException>(() => JsonScanner.ObjectMembers("[1]"));
        }

        [Fact]
        public void Strings_decode_every_escape()
        {
            Assert.Equal("\"\\/\b\f\n\r\t\u0001é😀", JsonScanner.DecodeString("\"\\\"\\\\\\/\\b\\f\\n\\r\\t\\u0001\\u00e9\\ud83d\\ude00\""));
            Assert.Equal("é😀", JsonScanner.DecodeString("\"é😀\""));
            Assert.Throws<FormatException>(() => JsonScanner.DecodeString("\"\\u12\""));
        }

        [Fact]
        public void Args_writer_output_parses_to_the_same_values()
        {
            var w = new ArgsWriter();
            w.Str("s", "q\"\\\u0001\n😀", "s");
            w.OptStr("absent", null);
            w.TriStr("cleared", FieldUpdate<string>.Clear);
            w.TriStr("left", FieldUpdate<string>.Leave);
            w.TriInt("tier", 3);
            w.Int("n", -9007199254740993);
            w.OptBool("b", true);
            w.Json("j", " {\"x\": [1]} ", "j");
            w.StrList("l", new[] { "a", "b" }, "l");
            JsonElement o = JsonDocument.Parse(w.ToJson()).RootElement;

            Assert.Equal("q\"\\\u0001\n😀", o.GetProperty("s").GetString());
            Assert.False(o.TryGetProperty("absent", out _));
            Assert.False(o.TryGetProperty("left", out _));
            Assert.Equal(JsonValueKind.Null, o.GetProperty("cleared").ValueKind);
            Assert.Equal(3, o.GetProperty("tier").GetInt64());
            Assert.Equal(-9007199254740993, o.GetProperty("n").GetInt64());
            Assert.True(o.GetProperty("b").GetBoolean());
            Assert.Equal(1, o.GetProperty("j").GetProperty("x")[0].GetInt32());
            Assert.Equal("b", o.GetProperty("l")[1].GetString());
            Assert.Equal("{}", new ArgsWriter().ToJson());
        }
    }
}
