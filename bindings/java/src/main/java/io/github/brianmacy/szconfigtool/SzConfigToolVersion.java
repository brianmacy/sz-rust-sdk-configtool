package io.github.brianmacy.szconfigtool;

/**
 * Version accessors of the loaded native library. Both values come from one
 * definition in the {@code sz-configtool-api} crate, shared by every binding
 * and the C ABI library.
 */
public final class SzConfigToolVersion {
    private SzConfigToolVersion() {
    }

    /**
     * The library version (the workspace version, for example {@code 4.4.0-1}).
     *
     * @return the library version string
     */
    public static String libraryVersion() {
        return NativeBridge.libraryVersion();
    }

    /**
     * The C ABI version this build implements ({@code SZCONFIGTOOL_ABI_VERSION}).
     *
     * @return the ABI version
     */
    public static int abiVersion() {
        return NativeBridge.abiVersion();
    }
}
