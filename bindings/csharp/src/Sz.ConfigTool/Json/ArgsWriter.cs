using System;
using System.Collections.Generic;
using System.Globalization;
using System.Text;

namespace Sz.ConfigTool.Json
{
    /// <summary>
    /// Builds the <c>args_json</c> object for <c>SzConfigTool_invoke</c>, keyed
    /// by the manifest's snake_case argument names. Absent optional values are
    /// omitted (leave / none); only tri-state <see cref="FieldUpdate{T}.Clear"/>
    /// writes <c>null</c>.
    /// </summary>
    internal sealed class ArgsWriter
    {
        private readonly StringBuilder _sb = new StringBuilder("{");
        private bool _any;

        /// <summary>The finished JSON object.</summary>
        public string ToJson() => _sb.ToString() + "}";

        public void Str(string key, string value, string param) => WriteString(key, NotNull(value, param));

        public void OptStr(string key, string? value)
        {
            if (value != null)
            {
                WriteString(key, value);
            }
        }

        public void TriStr(string key, FieldUpdate<string> value)
        {
            if (value.IsClear)
            {
                WriteRaw(key, "null");
            }
            else if (value.IsSet)
            {
                WriteString(key, value.Value);
            }
        }

        public void Int(string key, long value) => WriteRaw(key, value.ToString(CultureInfo.InvariantCulture));

        public void OptInt(string key, long? value)
        {
            if (value.HasValue)
            {
                Int(key, value.Value);
            }
        }

        public void TriInt(string key, FieldUpdate<long> value)
        {
            if (value.IsClear)
            {
                WriteRaw(key, "null");
            }
            else if (value.IsSet)
            {
                Int(key, value.Value);
            }
        }

        public void Bool(string key, bool value) => WriteRaw(key, value ? "true" : "false");

        public void OptBool(string key, bool? value)
        {
            if (value.HasValue)
            {
                Bool(key, value.Value);
            }
        }

        /// <summary>A raw JSON value; it must be exactly one valid JSON value.</summary>
        public void Json(string key, string rawJson, string param)
        {
            NotNull(rawJson, param);
            if (!JsonScanner.IsSingleValue(rawJson))
            {
                throw new ArgumentException("Must be exactly one valid JSON value.", param);
            }

            WriteRaw(key, rawJson.Trim(' ', '\t', '\n', '\r'));
        }

        public void OptJson(string key, string? rawJson, string param)
        {
            if (rawJson != null)
            {
                Json(key, rawJson, param);
            }
        }

        public void StrList(string key, IEnumerable<string> values, string param)
        {
            NotNull(values, param);
            var sb = new StringBuilder("[");
            bool first = true;
            foreach (string v in values)
            {
                if (v == null)
                {
                    throw new ArgumentException("List elements must not be null.", param);
                }

                if (!first)
                {
                    sb.Append(',');
                }

                AppendString(sb, v);
                first = false;
            }

            WriteRaw(key, sb.Append(']').ToString());
        }

        public void OptStrList(string key, IEnumerable<string>? values, string param)
        {
            if (values != null)
            {
                StrList(key, values, param);
            }
        }

        private static T NotNull<T>(T value, string param)
            where T : class =>
            value ?? throw new ArgumentNullException(param);

        private void WriteString(string key, string value)
        {
            var sb = new StringBuilder();
            AppendString(sb, value);
            WriteRaw(key, sb.ToString());
        }

        private void WriteRaw(string key, string rawValue)
        {
            if (_any)
            {
                _sb.Append(',');
            }

            AppendString(_sb, key);
            _sb.Append(':').Append(rawValue);
            _any = true;
        }

        /// <summary>Append <paramref name="value"/> as a JSON string literal.</summary>
        internal static void AppendString(StringBuilder sb, string value)
        {
            sb.Append('"');
            foreach (char c in value)
            {
                switch (c)
                {
                    case '"': sb.Append("\\\""); break;
                    case '\\': sb.Append("\\\\"); break;
                    case '\n': sb.Append("\\n"); break;
                    case '\r': sb.Append("\\r"); break;
                    case '\t': sb.Append("\\t"); break;
                    case '\b': sb.Append("\\b"); break;
                    case '\f': sb.Append("\\f"); break;
                    default:
                        if (c < 0x20)
                        {
                            sb.Append("\\u").Append(((int)c).ToString("x4", CultureInfo.InvariantCulture));
                        }
                        else
                        {
                            sb.Append(c);
                        }

                        break;
                }
            }

            sb.Append('"');
        }
    }
}
