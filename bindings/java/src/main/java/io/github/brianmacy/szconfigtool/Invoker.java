package io.github.brianmacy.szconfigtool;

import java.util.Map;
import java.util.Objects;

/** Shared call/unwrap helpers for the generated wrappers. */
final class Invoker {
    private Invoker() {
    }

    /** Invoke and check the result kind matches the manifest's {@code returns}. */
    static String[] call(String name, String expectedKind, String config, Args args)
            throws SzConfigToolException {
        Objects.requireNonNull(config, "configJson");
        String[] out = NativeBridge.invoke(name, config, args.toJson());
        if (out == null || out.length != 3 || !expectedKind.equals(out[0])) {
            String got = out == null || out.length == 0 ? "nothing" : out[0];
            throw new SzConfigToolException("INTERNAL", "INTERNAL",
                    name + ": expected result kind " + expectedKind + " but got " + got, null);
        }
        return out;
    }

    static String config(String name, String config, Args args) throws SzConfigToolException {
        return call(name, "config", config, args)[1];
    }

    static String json(String name, String config, Args args) throws SzConfigToolException {
        return call(name, "json", config, args)[2];
    }

    static ConfigAndJson configAndJson(String name, String config, Args args)
            throws SzConfigToolException {
        String[] out = call(name, "config_and_json", config, args);
        return new ConfigAndJson(out[1], out[2]);
    }

    static long integer(String name, String config, Args args) throws SzConfigToolException {
        return Long.parseLong(call(name, "int", config, args)[2]);
    }

    static void unit(String name, String config, Args args) throws SzConfigToolException {
        call(name, "unit", config, args);
    }

    /**
     * The named members of a {@code tuple_names} record, each as JSON text.
     *
     * @param record the record JSON object text
     * @param names member names in order
     * @return each member's JSON text
     */
    static String[] fields(String record, String... names) throws SzConfigToolException {
        Object parsed = Json.parse(record);
        if (!(parsed instanceof Map<?, ?> map)) {
            throw new SzConfigToolException("INTERNAL", "INTERNAL",
                    "expected a record object, got " + record, null);
        }
        String[] out = new String[names.length];
        for (int i = 0; i < names.length; i++) {
            if (!map.containsKey(names[i])) {
                throw new SzConfigToolException("INTERNAL", "INTERNAL",
                        "record lacks '" + names[i] + "': " + record, null);
            }
            out[i] = Json.write(map.get(names[i]));
        }
        return out;
    }
}
