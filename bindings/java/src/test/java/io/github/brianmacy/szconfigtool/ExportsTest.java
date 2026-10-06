package io.github.brianmacy.szconfigtool;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assumptions.assumeFalse;

import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.Locale;
import java.util.concurrent.TimeUnit;
import org.junit.jupiter.api.Test;

/** The JNI library exports only {@code Java_*} (+ {@code JNI_OnLoad}), never {@code SzConfigTool_*}. */
class ExportsTest {
    @Test
    void onlyJniSymbolsAreExported() throws Exception {
        String os = System.getProperty("os.name").toLowerCase(Locale.ROOT);
        assumeFalse(os.startsWith("windows"), "uses nm (dumpbin on Windows: not covered here)");
        Path classes = Path.of(NativeLoader.class.getProtectionDomain().getCodeSource()
                .getLocation().getPath());
        Path lib = classes.resolve("natives").resolve(TestSupport.prop("platform").toString())
                .resolve(TestSupport.prop("libFile").toString());
        assertTrue(Files.isRegularFile(lib), lib.toString());
        boolean mac = os.startsWith("mac");
        List<String> cmd = mac ? List.of("nm", "-gU", lib.toString())
                : List.of("nm", "-D", "--defined-only", lib.toString());
        Process p = new ProcessBuilder(cmd).redirectErrorStream(true).start();
        String out = new String(p.getInputStream().readAllBytes(), StandardCharsets.UTF_8);
        assertTrue(p.waitFor(60, TimeUnit.SECONDS));
        assertEquals(0, p.exitValue(), out);
        List<String> symbols = new ArrayList<>();
        for (String line : out.split("\n")) {
            String[] cols = line.trim().split("\\s+");
            if (cols.length >= 3) {
                String sym = cols[cols.length - 1];
                symbols.add(mac && sym.startsWith("_") ? sym.substring(1) : sym);
            }
        }
        assertTrue(symbols.contains("Java_io_github_brianmacy_szconfigtool_NativeBridge_invoke"),
                out);
        for (String s : symbols) {
            assertTrue(s.startsWith("Java_") || s.equals("JNI_OnLoad") || s.equals("JNI_OnUnload"),
                    "unexpected export " + s);
            assertTrue(!s.contains("SzConfigTool_"), "leaked C ABI symbol " + s);
        }
    }
}
