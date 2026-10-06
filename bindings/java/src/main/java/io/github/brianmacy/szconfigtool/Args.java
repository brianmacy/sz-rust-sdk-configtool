package io.github.brianmacy.szconfigtool;

import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;

/**
 * Builder of an {@code args_json} object: keys are manifest argument names,
 * values pre-rendered JSON fragments. Used by the generated wrappers and
 * their {@code *Options} classes.
 */
final class Args {
    private final Map<String, String> fragments = new LinkedHashMap<>();

    Args copy() {
        Args a = new Args();
        a.fragments.putAll(fragments);
        return a;
    }

    Args str(String key, String value) {
        StringBuilder sb = new StringBuilder();
        Json.quote(sb, Objects.requireNonNull(value, key));
        fragments.put(key, sb.toString());
        return this;
    }

    Args integer(String key, long value) {
        fragments.put(key, Long.toString(value));
        return this;
    }

    Args bool(String key, boolean value) {
        fragments.put(key, Boolean.toString(value));
        return this;
    }

    /** A {@code json} argument: validated as exactly one JSON value. */
    Args json(String key, String jsonText) {
        Json.parse(Objects.requireNonNull(jsonText, key));
        fragments.put(key, jsonText.strip());
        return this;
    }

    Args strList(String key, List<String> values) {
        Objects.requireNonNull(values, key);
        for (String v : values) {
            Objects.requireNonNull(v, key + " element");
        }
        fragments.put(key, Json.write(values));
        return this;
    }

    /** A tri-state {@code str} argument. */
    Args strUpdate(String key, FieldUpdate<String> update) {
        return update(key, update, v -> str(key, v));
    }

    /** A tri-state {@code int} argument. */
    Args intUpdate(String key, FieldUpdate<Long> update) {
        return update(key, update, v -> integer(key, v));
    }

    private <T> Args update(String key, FieldUpdate<T> update, java.util.function.Consumer<T> set) {
        switch (Objects.requireNonNull(update, key).getState()) {
            case LEAVE -> fragments.remove(key);
            case CLEAR -> fragments.put(key, "null");
            case SET -> set.accept(update.getValue());
            default -> throw new IllegalStateException(update.toString());
        }
        return this;
    }

    String toJson() {
        StringBuilder sb = new StringBuilder("{");
        boolean first = true;
        for (Map.Entry<String, String> e : fragments.entrySet()) {
            if (!first) {
                sb.append(',');
            }
            first = false;
            Json.quote(sb, e.getKey());
            sb.append(':').append(e.getValue());
        }
        return sb.append('}').toString();
    }
}
