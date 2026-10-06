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
