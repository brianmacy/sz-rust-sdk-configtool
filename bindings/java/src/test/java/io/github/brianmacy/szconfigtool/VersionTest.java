package io.github.brianmacy.szconfigtool;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import org.junit.jupiter.api.Test;

/** The version accessors return the single-source (sz-configtool-api) values. */
class VersionTest {
    /** {@code [workspace.package] version} of the workspace Cargo.toml. */
    static String workspaceVersion() throws IOException {
        Path cargoToml = Path.of(System.getProperty("szconfigtool.test.cargoToml"));
        List<String> lines = Files.readAllLines(cargoToml);
        boolean inPackage = false;
        for (String line : lines) {
            String t = line.trim();
            if (t.startsWith("[")) {
                inPackage = t.equals("[workspace.package]");
            } else if (inPackage && t.startsWith("version = ")) {
                return t.substring("version = ".length()).replace("\"", "");
            }
        }
        throw new IllegalStateException("no [workspace.package] version in " + cargoToml);
    }

    @Test
    void libraryVersionEqualsWorkspaceVersion() throws IOException {
        assertEquals(workspaceVersion(), SzConfigToolVersion.libraryVersion());
    }

    @Test
    void abiVersionIsTheCAbiVersion() {
        assertEquals(2, SzConfigToolVersion.abiVersion());
    }
}
