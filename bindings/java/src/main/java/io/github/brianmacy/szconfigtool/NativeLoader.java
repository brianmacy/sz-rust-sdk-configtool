package io.github.brianmacy.szconfigtool;

import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.UncheckedIOException;
import java.nio.file.AccessDeniedException;
import java.nio.file.FileAlreadyExistsException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardCopyOption;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.Arrays;
import java.util.HexFormat;
import java.util.Locale;
import java.util.Properties;

/**
 * Loads the JNI library ({@code szconfigtool_jni}) once per JVM.
 *
 * <p>Order:
 * <ol>
 *   <li>System property {@value #PATH_PROPERTY}: an absolute library file,
 *       loaded with {@link System#load} (explicit override).</li>
 *   <li>The bundled resource {@code /natives/<os>-<arch>/<lib>} (for example
 *       {@code natives/macos-aarch64/libszconfigtool_jni.dylib}), extracted
 *       to {@code <dir>/<version>-<sha256 prefix>/<lib>} and loaded. The file
 *       is written to a temp file in that directory and atomically renamed, so
 *       concurrent JVMs never see a partial file; an existing file with the
 *       same content is reused (this also covers a Windows DLL that another
 *       JVM holds locked). {@code <dir>} is {@value #DIR_PROPERTY} if set
 *       (use it when {@code java.io.tmpdir} is mounted {@code noexec}), else
 *       {@code ${java.io.tmpdir}/sz-configtool-jni-${user.name}}.</li>
 *   <li>{@link System#loadLibrary} with {@value #NAME_PROPERTY} (default
 *       {@value #DEFAULT_NAME}), searching {@code java.library.path}.</li>
 * </ol>
 */
final class NativeLoader {
    /** Absolute path of the library file to load instead of the bundled one. */
    static final String PATH_PROPERTY = "szconfigtool.native.path";
    /** Base directory for extracting the bundled library. */
    static final String DIR_PROPERTY = "szconfigtool.native.dir";
    /** Library name for the {@code System.loadLibrary} fallback. */
    static final String NAME_PROPERTY = "szconfigtool.native.name";
    /** The JNI crate's library name (bindings/jni/Cargo.toml {@code [lib] name}). */
    static final String DEFAULT_NAME = "szconfigtool_jni";
    /** Filtered by Maven: {@code version=${project.version}}. */
    private static final String VERSION_RESOURCE = "version.properties";
    private static final int HASH_HEX_CHARS = 16;

    private static String loadedFrom;

    private NativeLoader() {
    }

    /** Load the library (idempotent). */
    static synchronized void load() {
        if (loadedFrom != null) {
            return;
        }
        String explicit = System.getProperty(PATH_PROPERTY);
        if (explicit != null && !explicit.isBlank()) {
            System.load(Paths.get(explicit).toAbsolutePath().toString());
            loadedFrom = "path:" + explicit;
            return;
        }
        String resource = resourcePath(platform(System.getProperty("os.name"),
                System.getProperty("os.arch")), System.mapLibraryName(DEFAULT_NAME));
        try {
            byte[] bundled = resourceBytes(resource);
            if (bundled != null) {
                Path lib = extract(bundled, extractionBase(), version(),
                        System.mapLibraryName(DEFAULT_NAME));
                loadExtracted(lib);
                loadedFrom = "resource:" + lib;
                return;
            }
        } catch (IOException e) {
            throw new UncheckedIOException("extracting " + resource, e);
        }
        String name = System.getProperty(NAME_PROPERTY, DEFAULT_NAME);
        try {
            System.loadLibrary(name);
        } catch (UnsatisfiedLinkError e) {
            UnsatisfiedLinkError err = new UnsatisfiedLinkError("no bundled " + resource
                    + " and System.loadLibrary(\"" + name + "\") failed (set -D" + PATH_PROPERTY
                    + "=<file> or java.library.path): " + e.getMessage());
            err.initCause(e);
            throw err;
        }
        loadedFrom = "library:" + name;
    }

    /** @return how the library was loaded ({@code path:}, {@code resource:} or {@code library:}) */
    static synchronized String loadedFrom() {
        return loadedFrom;
    }

    private static void loadExtracted(Path lib) {
        try {
            System.load(lib.toString());
        } catch (UnsatisfiedLinkError e) {
            UnsatisfiedLinkError err = new UnsatisfiedLinkError("loading extracted " + lib
                    + " failed (if the directory is mounted noexec, set -D" + DIR_PROPERTY
                    + "=<exec-allowed dir>): " + e.getMessage());
            err.initCause(e);
            throw err;
        }
    }

    /**
     * The bundle platform directory for {@code os.name}/{@code os.arch}.
     *
     * @param osName {@code os.name}
     * @param osArch {@code os.arch}
     * @return for example {@code linux-x86_64}, {@code macos-aarch64}, {@code windows-x86_64}
     */
    static String platform(String osName, String osArch) {
        String os = osName.toLowerCase(Locale.ROOT);
        String osPart;
        if (os.startsWith("mac") || os.startsWith("darwin")) {
            osPart = "macos";
        } else if (os.startsWith("windows")) {
            osPart = "windows";
        } else {
            osPart = os.replaceAll("[^a-z0-9]", "");
        }
        String arch = osArch.toLowerCase(Locale.ROOT);
        String archPart = switch (arch) {
            case "amd64", "x86_64", "x64" -> "x86_64";
            case "aarch64", "arm64" -> "aarch64";
            default -> arch.replaceAll("[^a-z0-9_]", "");
        };
        return osPart + "-" + archPart;
    }

    static String resourcePath(String platform, String fileName) {
        return "/natives/" + platform + "/" + fileName;
    }

    static Path extractionBase() {
        String dir = System.getProperty(DIR_PROPERTY);
        if (dir != null && !dir.isBlank()) {
            return Paths.get(dir);
        }
        String user = System.getProperty("user.name", "user").replaceAll("[^A-Za-z0-9._-]", "_");
        return Paths.get(System.getProperty("java.io.tmpdir"), "sz-configtool-jni-" + user);
    }

    /**
     * The bytes of classpath resource {@code path}, or null when it is absent.
     * (A plain try/finally: try-with-resources on a nullable stream leaves a
     * javac null-check branch that can never be taken.)
     */
    static byte[] resourceBytes(String path) throws IOException {
        InputStream in = NativeLoader.class.getResourceAsStream(path);
        if (in == null) {
            return null;
        }
        try {
            return in.readAllBytes();
        } finally {
            in.close();
        }
    }

    static String version() {
        Properties p = new Properties();
        try {
            byte[] bytes = resourceBytes(VERSION_RESOURCE);
            if (bytes != null) {
                p.load(new ByteArrayInputStream(bytes));
            }
        } catch (IOException e) {
            throw new UncheckedIOException("reading " + VERSION_RESOURCE, e);
        }
        return p.getProperty("version", "dev").replaceAll("[^A-Za-z0-9._-]", "_");
    }

    /**
     * Write {@code bytes} to {@code base/<version>-<hash>/<fileName>} unless an
     * identical file is already there. Safe against concurrent extractors.
     *
     * @return the library file
     */
    static Path extract(byte[] bytes, Path base, String version, String fileName)
            throws IOException {
        return extract(bytes, base, version, fileName,
                (from, to) -> Files.move(from, to, StandardCopyOption.ATOMIC_MOVE));
    }

    /** The rename step of {@link #extract}, injectable so every OS can test its failure modes. */
    @FunctionalInterface
    interface Mover {
        void move(Path from, Path to) throws IOException;
    }

    static Path extract(byte[] bytes, Path base, String version, String fileName, Mover mover)
            throws IOException {
        Path dir = base.resolve(version + "-" + sha256(bytes).substring(0, HASH_HEX_CHARS));
        Files.createDirectories(dir);
        Path target = dir.resolve(fileName);
        if (sameContent(target, bytes)) {
            return target;
        }
        Path tmp = Files.createTempFile(dir, fileName, ".tmp");
        try {
            Files.write(tmp, bytes);
            mover.move(tmp, target);
        } catch (FileAlreadyExistsException | AccessDeniedException e) {
            // Another JVM won the race, or (Windows) holds the DLL loaded.
            if (!sameContent(target, bytes)) {
                throw new IOException("existing " + target + " differs from the bundled library", e);
            }
        } finally {
            Files.deleteIfExists(tmp);
        }
        return target;
    }

    private static boolean sameContent(Path file, byte[] bytes) throws IOException {
        return Files.isRegularFile(file) && Files.size(file) == bytes.length
                && Arrays.equals(Files.readAllBytes(file), bytes);
    }

    static String sha256(byte[] bytes) {
        try {
            return HexFormat.of().formatHex(MessageDigest.getInstance("SHA-256").digest(bytes));
        } catch (NoSuchAlgorithmException e) {
            throw new IllegalStateException("SHA-256 unavailable", e);
        }
    }
}
