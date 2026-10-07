package io.github.brianmacy.szconfigtool;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

import java.io.File;
import java.io.IOException;
import java.lang.management.ManagementFactory;
import java.nio.charset.StandardCharsets;
import java.nio.file.AccessDeniedException;
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
import java.util.function.Predicate;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Stream;
import java.util.zip.ZipEntry;
import java.util.zip.ZipOutputStream;
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

    @Test
    void platformNames() {
        assertEquals("macos-aarch64", NativeLoader.platform("Mac OS X", "aarch64"));
        assertEquals("macos-x86_64", NativeLoader.platform("Mac OS X", "x86_64"));
        assertEquals("linux-x86_64", NativeLoader.platform("Linux", "amd64"));
        assertEquals("linux-aarch64", NativeLoader.platform("Linux", "aarch64"));
        assertEquals("windows-x86_64", NativeLoader.platform("Windows 11", "amd64"));
        assertEquals("windows-aarch64", NativeLoader.platform("Windows Server 2022", "arm64"));
        assertEquals("macos-x86_64", NativeLoader.platform("Darwin", "x64"));
        // Unknown OS / arch names pass through, lowercased and sanitized.
        assertEquals("sunos-sparc_v9x",NativeLoader.platform("Sun OS", "SPARC_v9-x"));
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
        assertArrayEquals(Files.readAllBytes(TestSupport.bundledLib()), Files.readAllBytes(extracted));
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
        // So is one of the same size but different bytes.
        Files.writeString(first, "library Z");
        assertEquals(first, NativeLoader.extract(a, base, "1.0", "lib.so"));
        assertArrayEquals(a, Files.readAllBytes(first));
        assertNoTempFiles(base);
    }

    @Test
    void concurrentThreadsExtractOnce() throws Exception {
        Path base = scratch("threads");
        byte[] lib = Files.readAllBytes(TestSupport.bundledLib());
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
        Path lib = TestSupport.bundledLib();
        String out = runProbe(List.of("-D" + NativeLoader.PATH_PROPERTY + "=" + lib), classpath());
        assertTrue(out.startsWith("path:" + lib), out);
    }

    @Test
    void loadLibraryFallbackWithoutBundle() throws Exception {
        // Copy the classes WITHOUT natives/, then load from java.library.path.
        Path classes = copyClasses("no-bundle", rel -> rel.startsWith("natives"));
        String cp = classes + File.pathSeparator + testClasses();
        Path libDir = TestSupport.bundledLib().getParent();
        String out = runProbe(List.of("-Djava.library.path=" + libDir), cp);
        assertTrue(out.startsWith("library:" + NativeLoader.DEFAULT_NAME), out);
    }

    @Test
    void loadIsIdempotent() throws Exception {
        NativeBridge.invoke("list_data_sources", "{\"G2_CONFIG\":{\"CFG_DSRC\":[]}}", "{}");
        String first = NativeLoader.loadedFrom();
        NativeLoader.load();
        assertEquals(first, NativeLoader.loadedFrom());
    }

    @Test
    void defaultExtractionDirIsPerUserUnderTmpdir() throws Exception {
        // No -Dszconfigtool.native.dir (null) and blank overrides (ignored) both
        // extract to ${java.io.tmpdir}/sz-configtool-jni-<sanitized user.name>.
        Path tmp = scratch("tmpdir");
        Path expected = tmp.resolve("sz-configtool-jni-a_b_c");
        List<String> common = List.of("-Djava.io.tmpdir=" + tmp, "-Duser.name=a b/c");
        for (List<String> extra : List.of(List.<String>of(), List.of("-D" + NativeLoader.PATH_PROPERTY
                + "=", "-D" + NativeLoader.DIR_PROPERTY + "= "))) {
            List<String> jvm = new ArrayList<>(common);
            jvm.addAll(extra);
            String out = runProbe(jvm, classpath());
            assertTrue(out.startsWith("resource:" + expected), out);
        }
    }

    @Test
    void unusableExtractionDirIsAnUncheckedIoError() throws Exception {
        Path notADir = Files.writeString(scratch("not-a-dir").resolve("file"), "x");
        String out = runProbe(List.of("-D" + NativeLoader.DIR_PROPERTY + "=" + notADir),
                classpath(), "load");
        // The cause is the platform's java.nio.file error for "not a directory".
        assertTrue(out.startsWith("threw java.io.UncheckedIOException: extracting "
                + NativeLoader.resourcePath(TestSupport.prop("platform").toString(),
                        TestSupport.prop("libFile").toString())
                + "\ncause java.nio.file."), out);
    }

    @Test
    void failedLoadLibraryNamesBothSources() throws Exception {
        Path classes = copyClasses("no-bundle-bad-name", rel -> rel.startsWith("natives"));
        String cp = classes + File.pathSeparator + testClasses();
        Path libDir = TestSupport.bundledLib().getParent();
        String out = runProbe(List.of("-Djava.library.path=" + libDir,
                "-D" + NativeLoader.NAME_PROPERTY + "=no_such_szconfigtool_lib"), cp, "load");
        assertTrue(out.startsWith("threw java.lang.UnsatisfiedLinkError: no bundled "
                + NativeLoader.resourcePath(TestSupport.prop("platform").toString(),
                        TestSupport.prop("libFile").toString())
                + " and System.loadLibrary(\"no_such_szconfigtool_lib\") failed (set -D"
                + NativeLoader.PATH_PROPERTY + "=<file> or java.library.path): "), out);
        assertTrue(out.endsWith("\ncause java.lang.UnsatisfiedLinkError\n"), out);
    }

    @Test
    void unloadableBundledLibraryNamesTheExtractedFile() throws Exception {
        // A bundle whose library is not a shared object, and no version.properties
        // (the extraction directory falls back to "dev-<hash>").
        byte[] junk = "not a shared library".getBytes(StandardCharsets.UTF_8);
        Path classes = copyClasses("junk-bundle", rel -> rel.endsWith("version.properties"));
        Files.write(classes.resolve("natives").resolve(TestSupport.prop("platform").toString())
                .resolve(TestSupport.prop("libFile").toString()), junk);
        Path base = scratch("junk-extract");
        String cp = classes + File.pathSeparator + testClasses();
        String out = runProbe(List.of("-D" + NativeLoader.DIR_PROPERTY + "=" + base), cp, "load");
        Path lib = base.resolve("dev-" + NativeLoader.sha256(junk).substring(0, 16))
                .resolve(TestSupport.prop("libFile").toString());
        assertTrue(out.startsWith("threw java.lang.UnsatisfiedLinkError: loading extracted " + lib
                + " failed (if the directory is mounted noexec, set -D" + NativeLoader.DIR_PROPERTY
                + "=<exec-allowed dir>): "), out);
        assertTrue(out.endsWith("\ncause java.lang.UnsatisfiedLinkError\n"), out);
        assertArrayEquals(junk, Files.readAllBytes(lib));
    }

    @Test
    void unreadableVersionResourceIsAnUncheckedIoError() throws Exception {
        // A real jar, ahead of the classes on the class path, whose deflated
        // version.properties entry is corrupt: reading it fails mid-stream.
        Path jar = scratch("bad-version").resolve("bad-version.jar");
        String entry = NativeLoader.class.getPackageName().replace('.', '/') + "/version.properties";
        try (ZipOutputStream zip = new ZipOutputStream(Files.newOutputStream(jar))) {
            zip.putNextEntry(new ZipEntry(entry));
            zip.write("version=9.9.9\n".repeat(64).getBytes(StandardCharsets.UTF_8));
            zip.closeEntry();
        }
        byte[] bytes = Files.readAllBytes(jar);
        // Local header: 30 fixed bytes + name + extra; then the deflate stream,
        // whose first byte 0xFF declares the reserved block type 3.
        int nameLen = (bytes[26] & 0xFF) | (bytes[27] & 0xFF) << 8;
        int extraLen = (bytes[28] & 0xFF) | (bytes[29] & 0xFF) << 8;
        bytes[30 + nameLen + extraLen] = (byte) 0xFF;
        Files.write(jar, bytes);
        String out = runProbe(List.of(), jar + File.pathSeparator + classpath(), "version");
        assertEquals("threw java.io.UncheckedIOException: reading version.properties\n"
                + "cause java.util.zip.ZipException\n", out);
    }

    @Test
    void missingSha256ProviderIsAnIllegalState() throws Exception {
        // A JVM whose security configuration lists only SunJCE (no SHA-256 MessageDigest).
        Path props = Files.writeString(scratch("security").resolve("java.security"),
                "security.provider.1=SunJCE\n");
        String out = runProbe(List.of("-Djava.security.properties==" + props), classpath(),
                "sha256");
        assertEquals("threw java.lang.IllegalStateException: SHA-256 unavailable\n"
                + "cause java.security.NoSuchAlgorithmException\n", out);
    }

    @Test
    void probeReportIsLfDelimitedOnEveryOs() throws Exception {
        // The assertions above match "\n"; on Windows println would write "\r\n".
        String out = runProbe(List.of("-Dline.separator=\r\n"), classpath(), "sha256");
        assertEquals("ok " + NativeLoader.sha256(new byte[0]) + "\n", out);
    }

    @Test
    void extractRefusesADifferentUndeletableLibrary() throws Exception {
        // macOS: an ACL denying delete makes rename-over fail with EACCES
        // (AccessDeniedException). Linux has no unprivileged equivalent.
        assumeTrue(System.getProperty("os.name").startsWith("Mac"), "needs macOS ACLs");
        Path base = scratch("undeletable");
        byte[] lib = "library A".getBytes(StandardCharsets.UTF_8);
        Path target = NativeLoader.extract(lib, base, "1.0", "lib.so");
        Files.writeString(target, "tampered");
        chmodAcl("+a", target);
        try {
            IOException e = assertThrows(IOException.class,
                    () -> NativeLoader.extract(lib, base, "1.0", "lib.so"));
            assertEquals("existing " + target + " differs from the bundled library", e.getMessage());
            assertEquals(AccessDeniedException.class, e.getCause().getClass());
            assertEquals("tampered", Files.readString(target));
        } finally {
            chmodAcl("-a", target);
        }
        assertNoTempFiles(base);
    }

    @Test
    void jniFallsBackToRuntimeExceptionWhenTheExceptionClassIsMissing() throws Exception {
        // Without SzConfigToolException.class the JNI seam cannot construct it
        // and must still throw (a RuntimeException carrying reason and message).
        Path classes = copyClasses("no-exception-class",
                rel -> rel.endsWith("SzConfigToolException.class"));
        String cp = classes + File.pathSeparator + testClasses();
        String out = runProbe(List.of("-D" + NativeLoader.DIR_PROPERTY + "=" + scratch("fallback")),
                cp, "error");
        assertTrue(out.startsWith("threw java.lang.RuntimeException: NOT_FOUND: "), out);
        assertTrue(out.contains("NOPE"), out);
    }

    private static void chmodAcl(String op, Path file) throws Exception {
        Process p = new ProcessBuilder("chmod", op, "everyone deny delete", file.toString())
                .redirectErrorStream(true).start();
        String out = new String(p.getInputStream().readAllBytes(), StandardCharsets.UTF_8);
        assertTrue(p.waitFor(60, TimeUnit.SECONDS), "chmod timed out");
        assertEquals(0, p.exitValue(), out);
    }

    /** A copy of the main classes dir (with natives/), leaving out paths matching {@code skip}. */
    private static Path copyClasses(String name, Predicate<Path> skip) throws IOException {
        Path classes = scratch(name);
        Path src = TestSupport.codeSourceDir(NativeLoader.class);
        try (Stream<Path> s = Files.walk(src)) {
            for (Path p : s.toList()) {
                Path rel = src.relativize(p);
                if (skip.test(rel)) {
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
        return classes;
    }

    private static String classpath() {
        return System.getProperty("java.class.path");
    }

    /** The coverage agent of this JVM (JaCoCo under -Pcoverage), so child JVMs are measured too. */
    private static List<String> agents() {
        return ManagementFactory.getRuntimeMXBean().getInputArguments().stream()
                .filter(a -> a.startsWith("-javaagent:")).toList();
    }

    private static String testClasses() {
        return TestSupport.codeSourceDir(LoadProbe.class).toString();
    }

    private static String runProbe(List<String> jvmArgs, String cp, String... probeArgs)
            throws Exception {
        List<String> cmd = new ArrayList<>();
        cmd.add(Path.of(System.getProperty("java.home"), "bin", "java").toString());
        cmd.addAll(agents());
        cmd.addAll(jvmArgs);
        cmd.add("-cp");
        cmd.add(cp);
        cmd.add(LoadProbe.class.getName());
        cmd.addAll(List.of(probeArgs));
        // The probe reports on stdout only. stderr is kept apart (shown on failure):
        // the JVM writes its own diagnostics there, e.g. HotSpot on x86-64 Linux
        // warns "might have disabled stack guard" for any library without a
        // PT_GNU_STACK note, which includes the junk "library" of one test.
        Path err = Files.createTempFile(Files.createDirectories(TestSupport.prop("scratch")),
                "probe", ".stderr");
        Process p = new ProcessBuilder(cmd).redirectError(err.toFile()).start();
        String out = new String(p.getInputStream().readAllBytes(), StandardCharsets.UTF_8);
        assertTrue(p.waitFor(120, TimeUnit.SECONDS), "probe timed out");
        String stderr = Files.readString(err);
        Files.delete(err);
        assertEquals(0, p.exitValue(), out + "\nstderr:\n" + stderr);
        return out;
    }

    private static void assertNoTempFiles(Path base) throws IOException {
        try (Stream<Path> s = Files.walk(base)) {
            assertEquals(List.of(), s.filter(p -> p.toString().endsWith(".tmp")).toList());
        }
    }
}
