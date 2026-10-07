package io.github.brianmacy.szconfigtool;

import java.util.Objects;

/**
 * Result of a {@code config_and_json} function: the modified configuration
 * (opaque, byte-exact) and the record the library returned, as JSON text.
 *
 * @param config the modified configuration JSON document
 * @param json the record (for example the new row) as JSON text
 */
public record ConfigAndJson(String config, String json) {
    /**
     * @param config the modified configuration (non-null)
     * @param json the record JSON text (non-null)
     */
    public ConfigAndJson {
        Objects.requireNonNull(config, "config");
        Objects.requireNonNull(json, "json");
    }
}
