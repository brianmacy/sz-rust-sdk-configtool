package io.github.brianmacy.szconfigtool;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import java.math.BigDecimal;
import java.math.BigInteger;
import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

class JsonTest {
    @Test
    void parsesAndWritesAllShapes() {
        String text = "{\"a\":[1,-2,3.5,1e3,true,false,null],\"b\":\"q\\\"\\\\\\n\\u00e9\\ud83d\\ude00\","
                + "\"c\":{},\"d\":[],\"big\":123456789012345678901234567890}";
        Map<?, ?> m = (Map<?, ?>) Json.parse(" " + text + "\n");
        assertEquals(List.of(1L, -2L, new BigDecimal("3.5"), new BigDecimal("1e3"), true, false),
                ((List<?>) m.get("a")).subList(0, 6));
        assertEquals("q\"\\\né\uD83D\uDE00", m.get("b"));
        assertEquals(new BigInteger("123456789012345678901234567890"), m.get("big"));
        assertEquals("{\"a\":[1,-2,3.5,1E+3,true,false,null],\"b\":\"q\\\"\\\\\\né\uD83D\uDE00\","
                + "\"c\":{},\"d\":[],\"big\":123456789012345678901234567890}", Json.write(m));
        assertEquals("\"\\u0001\\t\"", Json.write("\u0001\t"));
    }

    @Test
    void rejectsInvalid() {
        for (String bad : new String[] {"", "{", "[1,]", "{\"a\" 1}", "tru", "1 2", "\"\\x\"",
                "-", "\"a", "{\"a\":1,}", "\"\\u12\""}) {
            assertThrows(IllegalArgumentException.class, () -> Json.parse(bad), bad);
        }
    }

    @Test
    void writesEveryNumberShapeAndRejectsNonJsonValues() {
        assertEquals("[7,8,1.5,2.5,3,true,9]", Json.write(List.of(7, 8L, 1.5d, 2.5f, (short) 3,
                Boolean.TRUE, BigInteger.valueOf(9))));
        IllegalArgumentException e = assertThrows(IllegalArgumentException.class,
                () -> Json.write(List.of(new Object())));
        assertEquals("not a JSON value: java.lang.Object", e.getMessage());
    }

    @Test
    void quotesEveryShortEscape() {
        assertEquals("\"\\\"\\\\\\n\\r\\t\\b\\f\\u001f/\"", Json.write("\"\\\n\r\t\b\f\u001f/"));
    }

    @Test
    void parsesEveryEscapeAndNumberForm() {
        assertEquals("\"\\/\n\r\t\b\fA", Json.parse("\"\\\"\\\\\\/\\n\\r\\t\\b\\f\\u0041\""));
        assertEquals(List.of(new BigDecimal("1E+2"), new BigDecimal("1e-2"), new BigDecimal("-0.5")),
                Json.parse("[1E+2,1e-2,-0.5]"));
    }

    @Test
    void rejectionMessagesNameTheFault() {
        Map<String, String> cases = Map.of(
                "\"a\u0001\"", "invalid JSON at offset 3: control character in string",
                "\"\\uZZZZ\"", "invalid JSON at offset 3: bad \\u escape",
                "1e", "invalid JSON at offset 2: bad number '1e'",
                "1.2.3", "invalid JSON at offset 5: bad number '1.2.3'",
                "x", "invalid JSON at offset 0: unexpected character");
        cases.forEach((bad, message) -> assertEquals(message,
                assertThrows(IllegalArgumentException.class, () -> Json.parse(bad), bad).getMessage()));
    }

    @Test
    void argsRendering() {
        Args a = new Args().str("s", "x\"y").integer("i", -3).bool("b", true)
                .strList("l", List.of("p", "q")).json("j", " {\"k\": [1]} ");
        assertEquals("{\"s\":\"x\\\"y\",\"i\":-3,\"b\":true,\"l\":[\"p\",\"q\"],\"j\":{\"k\": [1]}}",
                a.toJson());
        Args copy = a.copy().str("s", "changed");
        assertEquals(true, a.toJson().contains("x\\\"y"));
        assertEquals(true, copy.toJson().contains("changed"));
        assertThrows(NullPointerException.class, () -> new Args().strList("l",
                java.util.Arrays.asList("a", null)));
    }
}
