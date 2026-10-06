package io.github.brianmacy.szconfigtool;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.File;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.concurrent.Callable;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.concurrent.TimeUnit;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Stream;
import org.junit.jupiter.api.Test;

/** Bundled-resource extraction, overrides, and concurrent extraction. */
class NativeLoaderTest {
    private static final int CONCURRENCY = 8;

    private static Path scratch(String name) throws IOException {
        Path dir = TestSupport.prop("scratch").resolve(name);
        if (Files.exists(dir)) {
            try (Stream<Path> s = Files.walk(dir)) {
                for (Path p : s.sorted(Comparator.reverseOrder()).toList()) {
                    Files.delete(p);
                }
            }
        }
        return Files.createDirectories(dir);
    }

    private static Path bundledLib() {
        Path classes = Path.of(NativeLoader.class.getProtectionDomain().getCodeSource()
                .getLocation().getPath());
        return classes.resolve("natives").resolve(TestSupport.prop("platform").toString())
                .resolve(TestSupport.prop("libFile").toString());
    }

    @Test
    void platformNames() {
        assertEquals("macos-aarch64", NativeLoader.platform("Mac OS X", "aarch64"));
        assertEquals("macos-x86_64", NativeLoader.platform("Mac OS X", "x86_64"));
        assertEquals("linux-x86_64", NativeLoader.platform("Linux", "amd64"));
        assertEquals("linux-aarch64", NativeLoader.platform("Linux", "aarch64"));
        assertEquals("windows-x86_64", NativeLoader.platform("Windows 11", "amd64"));
        assertEquals("windows-aarch64", NativeLoader.platform("Windows Server 2022", "arm64"));
        assertEquals(TestSupport.prop("platform").toString(),
                NativeLoader.platform(System.getProperty("os.name"), System.getProperty("os.arch")));
        assertEquals("/natives/linux-x86_64/libszconfigtool_jni.so",
                NativeLoader.resourcePath("linux-x86_64", "libszconfigtool_jni.so"));
    }

    @Test
    void thisJvmLoadedTheBundledResource() throws Exception {
        NativeBridge.invoke("list_data_sources", "{\"G2_CONFIG\":{\"CFG_DSRC\":[]}}", "{}");
        String from = NativeLoader.loadedFrom();
        assertTrue(from.startsWith("resource:"), from);
        Path extracted = Path.of(from.substring("resource:".length()));
        assertTrue(extracted.startsWith(Path.of(System.getProperty(NativeLoader.DIR_PROPERTY))));
        assertArrayEquals(Files.readAllBytes(bundledLib()), Files.readAllBytes(extracted));
        String dir = extracted.getParent().getFileName().toString();
        assertEquals(NativeLoader.version() + "-"
                + NativeLoader.sha256(Files.readAllBytes(extracted)).substring(0, 16), dir);
    }

    @Test
    void versionMatchesCargoWorkspace() {
        Matcher m = Pattern.compile("(?ms)^\\[workspace\\.package\\].*?^version\\s*=\\s*\"([^\"]+)\"")
                .matcher(TestSupport.read(TestSupport.prop("cargoToml")));
        assertTrue(m.find());
        assertEquals(m.group(1), NativeLoader.version());
    }

    @Test
    void extractIsIdempotentAndContentAddressed() throws Exception {
        Path base = scratch("idempotent");
        byte[] a = "library A".getBytes(StandardCharsets.UTF_8);
        Path first = NativeLoader.extract(a, base, "1.0", "lib.so");
        Path again = NativeLoader.extract(a, base, "1.0", "lib.so");
        assertEquals(first, again);
        assertArrayEquals(a, Files.readAllBytes(first));
        Path other = NativeLoader.extract("library B".getBytes(StandardCharsets.UTF_8), base,
                "1.0", "lib.so");
        assertNotEquals(first.getParent(), other.getParent());
        // A corrupted file in the hashed directory is replaced atomically.
        Files.writeString(first, "corrupt");
        assertEquals(first, NativeLoader.extract(a, base, "1.0", "lib.so"));
        assertArrayEquals(a, Files.readAllBytes(first));
        assertNoTempFiles(base);
    }

    @Test
    void concurrentThreadsExtractOnce() throws Exception {
        Path base = scratch("threads");
        byte[] lib = Files.readAllBytes(bundledLib());
        ExecutorService pool = Executors.newFixedThreadPool(CONCURRENCY);
        try {
            List<Future<Path>> futures = new ArrayList<>();
            for (int i = 0; i < CONCURRENCY; i++) {
                futures.add(pool.submit(() -> NativeLoader.extract(lib, base, "t", "lib.bin")));
            }
            Set<Path> paths = new HashSet<>();
            for (Future<Path> f : futures) {
                paths.add(f.get(60, TimeUnit.SECONDS));
            }
            assertEquals(1, paths.size());
            assertArrayEquals(lib, Files.readAllBytes(paths.iterator().next()));
        } finally {
            pool.shutdownNow();
        }
        assertNoTempFiles(base);
    }

    @Test
    void concurrentJvmsExtractAndLoad() throws Exception {
        Path base = scratch("jvms");
        List<Callable<String>> jobs = new ArrayList<>();
        for (int i = 0; i < CONCURRENCY; i++) {
            jobs.add(() -> runProbe(List.of("-D" + NativeLoader.DIR_PROPERTY + "=" + base),
                    classpath()));
        }
        ExecutorService pool = Executors.newFixedThreadPool(CONCURRENCY);
        try {
            for (Future<String> f : pool.invokeAll(jobs)) {
                String out = f.get();
                assertTrue(out.startsWith("resource:" + base), out);
                assertTrue(out.contains("json []"), out);
            }
        } finally {
            pool.shutdownNow();
        }
        try (Stream<Path> s = Files.walk(base)) {
            assertEquals(1, s.filter(Files::isRegularFile).count());
        }
    }

    @Test
    void explicitPathOverrideWins() throws Exception {
        String out = runProbe(List.of("-D" + NativeLoader.PATH_PROPERTY + "=" + bundledLib()),
                classpath());
        assertTrue(out.startsWith("path:" + bundledLib()), out);
    }

    @Test
    void loadLibraryFallbackWithoutBundle() throws Exception {
        // Copy the classes WITHOUT natives/, then load from java.library.path.
        Path classes = scratch("no-bundle");
        Path src = Path.of(NativeLoader.class.getProtectionDomain().getCodeSource()
                .getLocation().getPath());
        try (Stream<Path> s = Files.walk(src)) {
            for (Path p : s.toList()) {
                Path rel = src.relativize(p);
                if (rel.startsWith("natives")) {
                    continue;
                }
                Path dest = classes.resolve(rel.toString());
                if (Files.isDirectory(p)) {
                    Files.createDirectories(dest);
                } else {
                    Files.copy(p, dest);
                }
            }
        }
        String cp = classes + File.pathSeparator + testClasses();
        String out = runProbe(List.of("-Djava.library.path=" + bundledLib().getParent()), cp);
        assertTrue(out.startsWith("library:" + NativeLoader.DEFAULT_NAME), out);
    }

    private static String classpath() {
        return System.getProperty("java.class.path");
    }

    private static String testClasses() {
        return Path.of(LoadProbe.class.getProtectionDomain().getCodeSource().getLocation()
                .getPath()).toString();
    }

    private static String runProbe(List<String> jvmArgs, String cp) throws Exception {
        List<String> cmd = new ArrayList<>();
        cmd.add(Path.of(System.getProperty("java.home"), "bin", "java").toString());
        cmd.addAll(jvmArgs);
        cmd.add("-cp");
        cmd.add(cp);
        cmd.add(LoadProbe.class.getName());
        Process p = new ProcessBuilder(cmd).redirectErrorStream(true).start();
        String out = new String(p.getInputStream().readAllBytes(), StandardCharsets.UTF_8);
        assertTrue(p.waitFor(120, TimeUnit.SECONDS), "probe timed out");
        assertEquals(0, p.exitValue(), out);
        return out;
    }

    private static void assertNoTempFiles(Path base) throws IOException {
        try (Stream<Path> s = Files.walk(base)) {
            assertEquals(List.of(), s.filter(p -> p.toString().endsWith(".tmp")).toList());
        }
    }
}
