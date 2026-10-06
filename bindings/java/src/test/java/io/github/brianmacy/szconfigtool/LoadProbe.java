package io.github.brianmacy.szconfigtool;

/** Child-JVM entry point for the loader tests: load, call once, report. */
public final class LoadProbe {
    private LoadProbe() {
    }

    public static void main(String[] args) throws Exception {
        String[] out = NativeBridge.invoke("list_data_sources",
                "{\"G2_CONFIG\":{\"CFG_DSRC\":[]}}", "{}");
        System.out.println(NativeLoader.loadedFrom());
        System.out.println(out[0] + " " + out[2]);
    }
}
