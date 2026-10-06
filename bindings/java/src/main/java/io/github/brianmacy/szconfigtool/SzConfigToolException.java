package io.github.brianmacy.szconfigtool;

/**
 * The one checked exception of this binding: every library and boundary
 * failure, classified by a stable reason code (see
 * {@code api/manifest/project.yaml} {@code reason_codes}).
 *
 * <p>Deliberately NOT a {@code com.senzing.sdk.SzException}: this library
 * edits configuration documents and has no engine semantics, and it does not
 * depend on {@code sz-sdk.jar}.
 */
public class SzConfigToolException extends Exception {
    private static final long serialVersionUID = 1L;

    private final SzConfigToolErrorKind kind;
    private final String reasonCode;
    private final String details;

    /**
     * Constructed by the native layer.
     *
     * @param kind the error kind (the reason code, as a string)
     * @param reasonCode the stable reason code, for example {@code NOT_FOUND}
     * @param message the human-readable message
     * @param details {@code sz-configtool.validation-errors/v1} JSON for
     *     {@code VALIDATION_ERRORS}, else {@code null}
     */
    public SzConfigToolException(String kind, String reasonCode, String message, String details) {
        super(message);
        this.kind = SzConfigToolErrorKind.fromCode(kind);
        this.reasonCode = reasonCode;
        this.details = details;
    }

    /**
     * @return the typed kind: the reason code itself as an enum constant
     *     ({@code getKind().name().equals(getReasonCode())}); {@code INTERNAL}
     *     only for a code this binding does not know
     */
    public SzConfigToolErrorKind getKind() {
        return kind;
    }

    /** @return the stable reason code string, for example {@code NOT_FOUND} */
    public String getReasonCode() {
        return reasonCode;
    }

    /**
     * @return the validation details JSON
     *     ({@code {"schema":"sz-configtool.validation-errors/v1","failures":[...]}})
     *     or {@code null} when the error carries none
     */
    public String getDetails() {
        return details;
    }
}
