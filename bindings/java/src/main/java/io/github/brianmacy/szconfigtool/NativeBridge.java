package io.github.brianmacy.szconfigtool;

/**
 * The single native seam: {@code invoke(name, config, argsJson)} over
 * {@code sz_configtool_api::invoke}. The generated {@link SzConfigTool}
 * methods are typed wrappers over it; call it directly only for dynamic use
 * (names and arguments are the manifest's snake_case names).
 *
 * <p>Loading the native library: see {@link NativeLoader}.
 */
public final class NativeBridge {
    static {
        NativeLoader.load();
    }

    private NativeBridge() {
    }

    /**
     * Call manifest function {@code name}.
     *
     * @param name manifest (snake_case) function name, for example {@code add_data_source}
     * @param config the configuration JSON document (opaque; passed unchanged)
     * @param argsJson JSON object keyed by manifest argument names; absent key =
     *     leave/none, {@code null} = clear (tri-state only), value = set.
     *     {@code null} means no arguments
     * @return {@code {kind, config, result}}: {@code kind} is the manifest
     *     {@code returns} value; {@code config} the modified configuration (or
     *     {@code null}); {@code result} the result as JSON text (or {@code null})
     * @throws SzConfigToolException on any failure ({@code INVALID_INPUT} for an
     *     unknown name or bad arguments, {@code MISSING_FIELD} for a missing
     *     required argument, the library's reason code otherwise)
     */
    public static native String[] invoke(String name, String config, String argsJson)
            throws SzConfigToolException;

    /**
     * The library version; use {@link SzConfigToolVersion#libraryVersion()}.
     *
     * @return the library version string
     */
    static native String libraryVersion();

    /**
     * The C ABI version; use {@link SzConfigToolVersion#abiVersion()}.
     *
     * @return the ABI version
     */
    static native int abiVersion();
}
