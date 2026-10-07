package io.github.brianmacy.szconfigtool;

import java.io.IOException;
import java.io.UncheckedIOException;
import java.net.URISyntaxException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;
import java.util.Map;

/** Paths (from surefire system properties set in pom.xml) and shared data. */
final class TestSupport {
    private TestSupport() {
    }

    static Path prop(String name) {
        String v = System.getProperty("szconfigtool.test." + name);
        if (v == null) {
            throw new IllegalStateException("system property szconfigtool.test." + name
                    + " not set (run through mvn)");
        }
        return Paths.get(v);
    }

    /**
     * The classes directory {@code c} was loaded from. Through the URI, not
     * {@code URL.getPath()}: on Windows that is {@code /D:/...}, not a file path
     * (and it keeps %-escapes such as {@code %20} everywhere).
     */
    static Path codeSourceDir(Class<?> c) {
        try {
            return Path.of(c.getProtectionDomain().getCodeSource().getLocation().toURI());
        } catch (URISyntaxException e) {
            throw new IllegalStateException(e);
        }
    }

    /** The JNI library bundled in the main classes dir ({@code natives/<platform>/<lib>}). */
    static Path bundledLib() {
        return codeSourceDir(NativeLoader.class).resolve("natives")
                .resolve(prop("platform").toString()).resolve(prop("libFile").toString());
    }

    static String read(Path p) {
        try {
            return Files.readString(p, StandardCharsets.UTF_8);
        } catch (IOException e) {
            throw new UncheckedIOException(e);
        }
    }

    @SuppressWarnings("unchecked")
    static Map<String, Object> conformance() {
        return (Map<String, Object>) Json.parse(read(prop("conformance")));
    }

    @SuppressWarnings("unchecked")
    static List<Map<String, Object>> manifestFunctions() {
        Map<String, Object> m = (Map<String, Object>) Json.parse(read(prop("manifest")));
        return (List<Map<String, Object>>) m.get("functions");
    }

    @SuppressWarnings("unchecked")
    static List<String> reasonCodes() {
        Map<String, Object> m = (Map<String, Object>) Json.parse(read(prop("manifest")));
        return (List<String>) m.get("reason_codes");
    }

    /** The REAL template configuration (conformance.json "fixture"). */
    static String fixture() {
        String rel = (String) conformance().get("fixture");
        return read(prop("workspace").resolve(rel));
    }
}
