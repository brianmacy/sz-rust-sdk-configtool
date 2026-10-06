package io.github.brianmacy.szconfigtool;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

class FieldUpdateTest {
    @Test
    void states() {
        FieldUpdate<String> leave = FieldUpdate.leave();
        FieldUpdate<String> clear = FieldUpdate.clear();
        FieldUpdate<String> set = FieldUpdate.set("v");
        assertTrue(leave.isLeave());
        assertTrue(clear.isClear());
        assertTrue(set.isSet());
        assertFalse(set.isLeave() || set.isClear());
        assertEquals("v", set.getValue());
        assertEquals(FieldUpdate.State.SET, set.getState());
        assertSame(FieldUpdate.<Long>leave(), FieldUpdate.<String>leave());
        assertThrows(IllegalStateException.class, leave::getValue);
        assertThrows(IllegalStateException.class, clear::getValue);
        assertThrows(NullPointerException.class, () -> FieldUpdate.set(null));
    }

    @Test
    void valueSemantics() {
        assertEquals(FieldUpdate.set(5L), FieldUpdate.set(5L));
        assertEquals(FieldUpdate.set(5L).hashCode(), FieldUpdate.set(5L).hashCode());
        assertNotEquals(FieldUpdate.set(5L), FieldUpdate.set(6L));
        assertNotEquals(FieldUpdate.clear(), FieldUpdate.leave());
        assertEquals("FieldUpdate.set(x)", FieldUpdate.set("x").toString());
        assertEquals("FieldUpdate.CLEAR", FieldUpdate.clear().toString());
        assertNotEquals(FieldUpdate.set("x"), "x");
        assertNotEquals(FieldUpdate.set(5L), FieldUpdate.clear());
    }

    @Test
    void predicatesAreExclusive() {
        for (FieldUpdate<String> u : java.util.List.of(FieldUpdate.<String>leave(),
                FieldUpdate.<String>clear(), FieldUpdate.set("v"))) {
            FieldUpdate.State s = u.getState();
            assertEquals(s == FieldUpdate.State.LEAVE, u.isLeave(), s.name());
            assertEquals(s == FieldUpdate.State.CLEAR, u.isClear(), s.name());
            assertEquals(s == FieldUpdate.State.SET, u.isSet(), s.name());
        }
    }

    @Test
    void wireEncoding() {
        Args a = new Args().strUpdate("s", FieldUpdate.set("v")).intUpdate("i", FieldUpdate.clear())
                .strUpdate("l", FieldUpdate.leave());
        assertEquals("{\"s\":\"v\",\"i\":null}", a.toJson());
        assertEquals("{\"s\":\"v\"}", a.intUpdate("i", FieldUpdate.leave()).toJson());
        assertEquals("{\"n\":42}", new Args().intUpdate("n", FieldUpdate.set(42L)).toJson());
    }
}
