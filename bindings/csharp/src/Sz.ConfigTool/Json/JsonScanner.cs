using System;
using System.Collections.Generic;
using System.Globalization;
using System.Text;

namespace Sz.ConfigTool.Json
{
    /// <summary>
    /// A minimal, strict (RFC 8259) JSON scanner. It never builds an object
    /// model: it validates and skips values, returns the members of a
    /// top-level object as raw text spans, and decodes string literals. That is
    /// all the envelope reader and the raw-JSON argument check need, and it
    /// keeps the binding dependency-free.
    /// </summary>
    internal sealed class JsonScanner
    {
        private const int MaxDepth = 512;
        private readonly string _text;
        private int _pos;

        private JsonScanner(string text)
        {
            _text = text;
        }

        /// <summary>True if <paramref name="text"/> is exactly one JSON value (surrounding whitespace allowed).</summary>
        public static bool IsSingleValue(string text)
        {
            var s = new JsonScanner(text);
            try
            {
                s.SkipWhitespace();
                s.SkipValue(0);
                s.SkipWhitespace();
                return s._pos == text.Length;
            }
            catch (FormatException)
            {
                return false;
            }
        }

        /// <summary>
        /// The members of the top-level object <paramref name="text"/>, each
        /// value as its exact raw JSON text.
        /// </summary>
        /// <exception cref="FormatException">Not a single JSON object, or a duplicate key.</exception>
        public static IReadOnlyDictionary<string, string> ObjectMembers(string text)
        {
            var s = new JsonScanner(text);
            var members = new Dictionary<string, string>(StringComparer.Ordinal);
            s.SkipWhitespace();
            s.Expect('{');
            s.SkipWhitespace();
            if (s.TryConsume('}'))
            {
                s.ExpectEnd();
                return members;
            }

            do
            {
                s.SkipWhitespace();
                string key = s.ReadString();
                s.SkipWhitespace();
                s.Expect(':');
                s.SkipWhitespace();
                int start = s._pos;
                s.SkipValue(1);
                if (members.ContainsKey(key))
                {
                    throw new FormatException($"duplicate key '{key}'");
                }

                members[key] = text.Substring(start, s._pos - start);
                s.SkipWhitespace();
            }
            while (s.TryConsume(','));

            s.Expect('}');
            s.ExpectEnd();
            return members;
        }

        /// <summary>Decode the raw JSON string literal <paramref name="literal"/> (with quotes).</summary>
        /// <exception cref="FormatException">Not exactly one JSON string.</exception>
        public static string DecodeString(string literal)
        {
            var s = new JsonScanner(literal);
            string value = s.ReadString();
            s.ExpectEnd();
            return value;
        }

        private void ExpectEnd()
        {
            SkipWhitespace();
            if (_pos != _text.Length)
            {
                throw Error("trailing characters");
            }
        }

        private FormatException Error(string what) =>
            new FormatException($"invalid JSON at offset {_pos}: {what}");

        private char Peek() => _pos < _text.Length ? _text[_pos] : '\0';

        private bool AtEnd => _pos >= _text.Length;

        private void Expect(char c)
        {
            if (AtEnd || _text[_pos] != c)
            {
                throw Error($"expected '{c}'");
            }

            _pos++;
        }

        private bool TryConsume(char c)
        {
            if (!AtEnd && _text[_pos] == c)
            {
                _pos++;
                return true;
            }

            return false;
        }

        private void SkipWhitespace()
        {
            while (!AtEnd && (_text[_pos] == ' ' || _text[_pos] == '\t' || _text[_pos] == '\n' || _text[_pos] == '\r'))
            {
                _pos++;
            }
        }

        private void SkipValue(int depth)
        {
            if (depth > MaxDepth)
            {
                throw Error("nesting too deep");
            }

            switch (Peek())
            {
                case '{':
                    SkipContainer('}', depth, true);
                    break;
                case '[':
                    SkipContainer(']', depth, false);
                    break;
                case '"':
                    ReadString();
                    break;
                case 't':
                    ExpectLiteral("true");
                    break;
                case 'f':
                    ExpectLiteral("false");
                    break;
                case 'n':
                    ExpectLiteral("null");
                    break;
                default:
                    SkipNumber();
                    break;
            }
        }

        private void SkipContainer(char close, int depth, bool isObject)
        {
            _pos++;
            SkipWhitespace();
            if (TryConsume(close))
            {
                return;
            }

            do
            {
                SkipWhitespace();
                if (isObject)
                {
                    ReadString();
                    SkipWhitespace();
                    Expect(':');
                    SkipWhitespace();
                }

                SkipValue(depth + 1);
                SkipWhitespace();
            }
            while (TryConsume(','));

            Expect(close);
        }

        private void ExpectLiteral(string literal)
        {
            if (string.CompareOrdinal(_text, _pos, literal, 0, literal.Length) != 0)
            {
                throw Error($"expected '{literal}'");
            }

            _pos += literal.Length;
        }

        private void SkipDigits()
        {
            int start = _pos;
            while (!AtEnd && _text[_pos] >= '0' && _text[_pos] <= '9')
            {
                _pos++;
            }

            if (_pos == start)
            {
                throw Error("expected a digit");
            }
        }

        private void SkipNumber()
        {
            TryConsume('-');
            if (!TryConsume('0'))
            {
                SkipDigits();
            }

            if (TryConsume('.'))
            {
                SkipDigits();
            }

            if (Peek() == 'e' || Peek() == 'E')
            {
                _pos++;
                if (!TryConsume('+'))
                {
                    TryConsume('-');
                }

                SkipDigits();
            }
        }

        private string ReadString()
        {
            Expect('"');
            var sb = new StringBuilder();
            while (true)
            {
                if (AtEnd)
                {
                    throw Error("unterminated string");
                }

                char c = _text[_pos++];
                if (c == '"')
                {
                    return sb.ToString();
                }

                if (c < 0x20)
                {
                    throw Error("control character in string");
                }

                if (c == '\\')
                {
                    sb.Append(ReadEscape());
                }
                else
                {
                    sb.Append(c);
                }
            }
        }

        private char ReadEscape()
        {
            if (AtEnd)
            {
                throw Error("unterminated escape");
            }

            char e = _text[_pos++];
            switch (e)
            {
                case '"': return '"';
                case '\\': return '\\';
                case '/': return '/';
                case 'b': return '\b';
                case 'f': return '\f';
                case 'n': return '\n';
                case 'r': return '\r';
                case 't': return '\t';
                case 'u': return ReadHex4();
                default: throw Error($"invalid escape '\\{e}'");
            }
        }

        // A \uXXXX escape yields one UTF-16 unit; surrogate pairs are two
        // consecutive escapes and recombine naturally in the StringBuilder.
        private char ReadHex4()
        {
            if (_pos + 4 > _text.Length ||
                !ushort.TryParse(_text.Substring(_pos, 4), NumberStyles.AllowHexSpecifier, CultureInfo.InvariantCulture, out ushort code))
            {
                throw Error("invalid \\u escape");
            }

            _pos += 4;
            return (char)code;
        }
    }
}
