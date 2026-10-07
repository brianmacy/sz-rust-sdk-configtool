package io.github.brianmacy.szconfigtool;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

/** Converters used by the generated {@link TypedDispatch}. */
final class Conv {
    private Conv() {
    }

    static String str(Object v) {
        return (String) v;
    }

    static long lng(Object v) {
        return (Long) v;
    }

    static boolean bool(Object v) {
        return (Boolean) v;
    }

    /** A {@code json} arg: the parsed conformance value re-rendered as JSON text. */
    static String json(Object v) {
        return Json.write(v);
    }

    static List<String> strList(Object v) {
        List<String> out = new ArrayList<>();
        for (Object o : (List<?>) v) {
            out.add((String) o);
        }
        return out;
    }

    static FieldUpdate<String> strUpdate(Object v) {
        return v == null ? FieldUpdate.clear() : FieldUpdate.set((String) v);
    }

    static FieldUpdate<Long> intUpdate(Object v) {
        return v == null ? FieldUpdate.clear() : FieldUpdate.set((Long) v);
    }

    static String[] config(String config) {
        return new String[] {"config", config, null};
    }

    static String[] json(String result) {
        return new String[] {"json", null, result};
    }

    static String[] jsonResult(String result) {
        return json(result);
    }

    /** A typed call that may throw the binding's checked exception. */
    @FunctionalInterface
    interface Call {
        String get() throws SzConfigToolException;
    }

    /**
     * A {@code config_and_json} step through BOTH typed methods: the primary
     * (the config) and the companion {@code <name>Result} (the record JSON).
     * When either fails, the other must fail with the same reason code.
     */
    static String[] configAndJson(Call primary, Call companion) throws SzConfigToolException {
        String config;
        try {
            config = primary.get();
        } catch (SzConfigToolException e) {
            SzConfigToolException other = assertThrows(SzConfigToolException.class, companion::get,
                    "companion must fail like the primary");
            assertEquals(e.getReasonCode(), other.getReasonCode());
            throw e;
        }
        return new String[] {"config_and_json", config, companion.get()};
    }

    static String[] integer(long v) {
        return new String[] {"int", null, Long.toString(v)};
    }

    static String[] unit() {
        return new String[] {"unit", null, null};
    }

    /** Rebuild a {@code tuple_names} record object from its JSON-text fields. */
    static String record(String[] names, String... jsonFields) {
        Map<String, Object> m = new LinkedHashMap<>();
        for (int i = 0; i < names.length; i++) {
            m.put(names[i], Json.parse(jsonFields[i]));
        }
        return Json.write(m);
    }
}
