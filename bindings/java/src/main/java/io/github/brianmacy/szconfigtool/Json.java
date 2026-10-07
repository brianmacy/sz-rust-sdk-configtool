package io.github.brianmacy.szconfigtool;

import java.math.BigDecimal;
import java.math.BigInteger;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

/**
 * A tiny, dependency-free JSON reader/writer for argument objects and result
 * values. It is NEVER used on configuration documents, which stay opaque
 * strings end to end.
 *
 * <p>Parsed values: {@code Map<String,Object>} (insertion ordered),
 * {@code List<Object>}, {@code String}, {@code Long} (integers that fit),
 * {@code BigInteger}, {@code BigDecimal}, {@code Boolean}, or {@code null}.
 */
final class Json {
    private final String text;
    private int pos;

    private Json(String text) {
        this.text = text;
    }

    /**
     * Parse exactly one JSON value.
     *
     * @param text JSON text
     * @return the parsed value
     * @throws IllegalArgumentException if {@code text} is not one valid JSON value
     */
    static Object parse(String text) {
        Json p = new Json(text);
        p.skipWs();
        Object value = p.value();
        p.skipWs();
        if (p.pos != text.length()) {
            throw p.error("trailing characters");
        }
        return value;
    }

    /**
     * Serialize a value of the shapes produced by {@link #parse} (plus any
     * {@code Number} and {@code Iterable}).
     *
     * @param value the value
     * @return compact JSON text
     */
    static String write(Object value) {
        StringBuilder sb = new StringBuilder();
        write(sb, value);
        return sb.toString();
    }

    static void write(StringBuilder sb, Object value) {
        if (value == null) {
            sb.append("null");
        } else if (value instanceof String s) {
            quote(sb, s);
        } else if (value instanceof Boolean || value instanceof Long || value instanceof Integer
                || value instanceof BigInteger) {
            sb.append(value);
        } else if (value instanceof BigDecimal d) {
            sb.append(d.toString());
        } else if (value instanceof Number n) {
            sb.append(new BigDecimal(n.toString()).toString());
        } else if (value instanceof Map<?, ?> m) {
            writeObject(sb, m);
        } else if (value instanceof Iterable<?> it) {
            writeArray(sb, it);
        } else {
            throw new IllegalArgumentException("not a JSON value: " + value.getClass().getName());
        }
    }

    private static void writeObject(StringBuilder sb, Map<?, ?> m) {
        sb.append('{');
        boolean first = true;
        for (Map.Entry<?, ?> e : m.entrySet()) {
            if (!first) {
                sb.append(',');
            }
            first = false;
            quote(sb, String.valueOf(e.getKey()));
            sb.append(':');
            write(sb, e.getValue());
        }
        sb.append('}');
    }

    private static void writeArray(StringBuilder sb, Iterable<?> it) {
        sb.append('[');
        boolean first = true;
        for (Object o : it) {
            if (!first) {
                sb.append(',');
            }
            first = false;
            write(sb, o);
        }
        sb.append(']');
    }

    /**
     * Append {@code s} as a JSON string literal.
     *
     * @param sb destination
     * @param s the string (UTF-16; surrogate pairs pass through unchanged)
     */
    static void quote(StringBuilder sb, String s) {
        sb.append('"');
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            switch (c) {
                case '"' -> sb.append("\\\"");
                case '\\' -> sb.append("\\\\");
                case '\n' -> sb.append("\\n");
                case '\r' -> sb.append("\\r");
                case '\t' -> sb.append("\\t");
                case '\b' -> sb.append("\\b");
                case '\f' -> sb.append("\\f");
                default -> {
                    if (c < 0x20) {
                        sb.append(String.format("\\u%04x", (int) c));
                    } else {
                        sb.append(c);
                    }
                }
            }
        }
        sb.append('"');
    }

    private IllegalArgumentException error(String what) {
        return new IllegalArgumentException("invalid JSON at offset " + pos + ": " + what);
    }

    private void skipWs() {
        while (pos < text.length() && " \t\r\n".indexOf(text.charAt(pos)) >= 0) {
            pos++;
        }
    }

    private char peek() {
        if (pos >= text.length()) {
            throw error("unexpected end");
        }
        return text.charAt(pos);
    }

    private void expect(char c) {
        if (peek() != c) {
            throw error("expected '" + c + "'");
        }
        pos++;
    }

    private Object value() {
        char c = peek();
        return switch (c) {
            case '{' -> object();
            case '[' -> array();
            case '"' -> string();
            case 't' -> literal("true", Boolean.TRUE);
            case 'f' -> literal("false", Boolean.FALSE);
            case 'n' -> literal("null", null);
            default -> number();
        };
    }

    private Object literal(String word, Object value) {
        if (!text.startsWith(word, pos)) {
            throw error("expected " + word);
        }
        pos += word.length();
        return value;
    }

    private Map<String, Object> object() {
        expect('{');
        Map<String, Object> m = new LinkedHashMap<>();
        skipWs();
        if (peek() == '}') {
            pos++;
            return m;
        }
        while (true) {
            skipWs();
            String key = string();
            skipWs();
            expect(':');
            skipWs();
            m.put(key, value());
            skipWs();
            if (peek() == ',') {
                pos++;
                continue;
            }
            expect('}');
            return m;
        }
    }

    private List<Object> array() {
        expect('[');
        List<Object> list = new ArrayList<>();
        skipWs();
        if (peek() == ']') {
            pos++;
            return list;
        }
        while (true) {
            skipWs();
            list.add(value());
            skipWs();
            if (peek() == ',') {
                pos++;
                continue;
            }
            expect(']');
            return list;
        }
    }

    private String string() {
        expect('"');
        StringBuilder sb = new StringBuilder();
        while (true) {
            char c = peek();
            pos++;
            if (c == '"') {
                return sb.toString();
            }
            if (c < 0x20) {
                throw error("control character in string");
            }
            if (c != '\\') {
                sb.append(c);
                continue;
            }
            char e = peek();
            pos++;
            switch (e) {
                case '"', '\\', '/' -> sb.append(e);
                case 'n' -> sb.append('\n');
                case 'r' -> sb.append('\r');
                case 't' -> sb.append('\t');
                case 'b' -> sb.append('\b');
                case 'f' -> sb.append('\f');
                case 'u' -> sb.append(unicodeEscape());
                default -> throw error("bad escape");
            }
        }
    }

    private char unicodeEscape() {
        if (pos + 4 > text.length()) {
            throw error("short \\u escape");
        }
        try {
            char c = (char) Integer.parseInt(text.substring(pos, pos + 4), 16);
            pos += 4;
            return c;
        } catch (NumberFormatException ex) {
            throw error("bad \\u escape");
        }
    }

    private Object number() {
        int start = pos;
        if (peek() == '-') {
            pos++;
        }
        boolean integral = true;
        while (pos < text.length()) {
            char c = text.charAt(pos);
            if (c >= '0' && c <= '9') {
                pos++;
            } else if (c == '.' || c == 'e' || c == 'E' || c == '+' || c == '-') {
                integral = false;
                pos++;
            } else {
                break;
            }
        }
        String lexeme = text.substring(start, pos);
        if (lexeme.isEmpty() || lexeme.equals("-")) {
            throw error("unexpected character");
        }
        try {
            if (!integral) {
                return new BigDecimal(lexeme);
            }
            BigInteger big = new BigInteger(lexeme);
            return big.bitLength() < 64 ? (Object) big.longValue() : big;
        } catch (NumberFormatException ex) {
            throw error("bad number '" + lexeme + "'");
        }
    }
}
