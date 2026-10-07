package io.github.brianmacy.szconfigtool;

import java.util.concurrent.Callable;

/**
 * Child-JVM entry point for the loader tests. With no argument: load, call
 * once, report. With a mode argument: run that one step and print
 * {@code ok <result>} or {@code threw <class>: <message>} (plus
 * {@code cause <class>} when there is one); the exit code is 0 either way.
 *
 * <p>Lines end in {@code \n} on every OS (not {@code line.separator}, which
 * is {@code \r\n} on Windows): the tests match the report exactly.
 *
 * <p>Only {@link Throwable} is caught, so this class never links
 * {@link SzConfigToolException} (one test runs without that class).
 */
public final class LoadProbe {
    private static final String CFG = "{\"G2_CONFIG\":{\"CFG_DSRC\":[]}}";

    private LoadProbe() {
    }

    private static void report(String line) {
        System.out.print(line + "\n");
        System.out.flush();
    }

    public static void main(String[] args) throws Exception {
        if (args.length == 0) {
            String[] out = NativeBridge.invoke("list_data_sources", CFG, "{}");
            report(NativeLoader.loadedFrom());
            report(out[0] + " " + out[2]);
            return;
        }
        Callable<Object> step = switch (args[0]) {
            case "load" -> () -> {
                NativeLoader.load();
                return NativeLoader.loadedFrom();
            };
            case "error" -> () -> NativeBridge.invoke("get_data_source", CFG, "{\"code\":\"NOPE\"}");
            case "version" -> NativeLoader::version;
            case "sha256" -> () -> NativeLoader.sha256(new byte[0]);
            default -> throw new IllegalArgumentException("unknown mode " + args[0]);
        };
        try {
            report("ok " + step.call());
        } catch (Throwable t) {
            report("threw " + t.getClass().getName() + ": " + t.getMessage());
            if (t.getCause() != null) {
                report("cause " + t.getCause().getClass().getName());
            }
        }
    }
}
