package io.github.brianmacy.szconfigtool;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import java.util.Map;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;

/** {@link Invoker}'s result-kind check and record unpacking, against the real library. */
class InvokerTest {
    private static final String CFG = "{\"G2_CONFIG\":{\"CFG_DSRC\":[]}}";
    /** A real function whose manifest {@code returns} is {@code json}. */
    private static final String JSON_FN = "list_data_sources";

    private static void internal(String message, Executable call) {
        SzConfigToolException e = assertThrows(SzConfigToolException.class, call);
        assertEquals("INTERNAL", e.getReasonCode());
        assertEquals(SzConfigToolErrorKind.INTERNAL, e.getKind());
        assertEquals(message, e.getMessage());
    }

    @Test
    void everyHelperRejectsAResultOfAnotherKind() {
        Map<String, Executable> helpers = Map.of(
                "config", () -> Invoker.config(JSON_FN, CFG, new Args()),
                "config_and_json", () -> Invoker.configAndJson(JSON_FN, CFG, new Args()),
                "int", () -> Invoker.integer(JSON_FN, CFG, new Args()),
                "unit", () -> Invoker.unit(JSON_FN, CFG, new Args()));
        helpers.forEach((kind, call) -> internal(
                JSON_FN + ": expected result kind " + kind + " but got json", call));
    }

    @Test
    void matchingKindReturnsTheNativeTriple() throws Exception {
        String[] out = Invoker.call(JSON_FN, "json", CFG, new Args());
        assertArrayEquals(new String[] {"json", null, "[]"}, out);
        assertEquals("[]", Invoker.json(JSON_FN, CFG, new Args()));
    }

    @Test
    void nullConfigIsRejectedBeforeTheNativeCall() {
        NullPointerException e = assertThrows(NullPointerException.class,
                () -> Invoker.json(JSON_FN, null, new Args()));
        assertEquals("configJson", e.getMessage());
    }

    @Test
    void fieldsUnpacksNamedRecordMembers() throws Exception {
        assertArrayEquals(new String[] {"[2,\"x\"]", "1", "null"},
                Invoker.fields("{\"a\":1,\"b\":[2,\"x\"],\"c\":null}", "b", "a", "c"));
        internal("expected a record object, got [1]", () -> Invoker.fields("[1]", "a"));
        internal("record lacks 'z': {\"a\":1}", () -> Invoker.fields("{\"a\":1}", "a", "z"));
    }
}
