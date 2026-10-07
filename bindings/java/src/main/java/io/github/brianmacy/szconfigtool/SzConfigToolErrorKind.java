// GENERATED — do not edit. Source: api/manifest/*.yaml; regenerate with `cargo run -p sz-configtool-codegen`.

package io.github.brianmacy.szconfigtool;

/** Every wire reason code ({@code api/manifest/project.yaml} {@code reason_codes}). */
public enum SzConfigToolErrorKind {
    /** {@code JSON_PARSE}. */
    JSON_PARSE,
    /** {@code NOT_FOUND}. */
    NOT_FOUND,
    /** {@code NOT_ON_CALL}. */
    NOT_ON_CALL,
    /** {@code NOT_IN_FEATURE}. */
    NOT_IN_FEATURE,
    /** {@code ALREADY_EXISTS}. */
    ALREADY_EXISTS,
    /** {@code ALREADY_PRESENT}. */
    ALREADY_PRESENT,
    /** {@code INVALID_INPUT}. */
    INVALID_INPUT,
    /** {@code MISSING_SECTION}. */
    MISSING_SECTION,
    /** {@code INVALID_STRUCTURE}. */
    INVALID_STRUCTURE,
    /** {@code MISSING_FIELD}. */
    MISSING_FIELD,
    /** {@code INVALID_CONFIG}. */
    INVALID_CONFIG,
    /** {@code NOT_IMPLEMENTED}. */
    NOT_IMPLEMENTED,
    /** {@code VALIDATION_ERRORS}. */
    VALIDATION_ERRORS,
    /** {@code INTERNAL}. */
    INTERNAL;

    /**
     * The kind for a reason code.
     *
     * @param code a reason code
     * @return its kind, or {@code INTERNAL} when unrecognized
     */
    public static SzConfigToolErrorKind fromCode(String code) {
        for (SzConfigToolErrorKind k : values()) {
            if (k.name().equals(code)) {
                return k;
            }
        }
        return INTERNAL;
    }
}
