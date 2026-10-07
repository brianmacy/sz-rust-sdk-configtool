package io.github.brianmacy.szconfigtool;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.fail;

import java.math.BigDecimal;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Set;
import java.util.TreeSet;
import java.util.stream.Stream;
import org.junit.jupiter.api.DynamicNode;
import org.junit.jupiter.api.DynamicTest;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestFactory;

/**
 * Runs every case of {@code api/manifest/generated/conformance.json} against
 * the REAL native library, through the GENERATED typed wrappers
 * ({@link TypedDispatch}). Steps flagged {@code wire_only} and steps calling
 * {@code not_implemented} functions (no typed wrapper) go through
 * {@link NativeBridge#invoke}. Matching rules: {@code api/manifest/schema.md}.
 */
class ConformanceTest {
    private static final Set<String> EXPECTED_ERRORS = new TreeSet<>();
    private static final Set<String> OBSERVED_TYPED_ERRORS = new TreeSet<>();
    private static int typedSteps;
    private static int wireSteps;

    @TestFactory
    @SuppressWarnings("unchecked")
    Stream<DynamicNode> conformance() {
        Map<String, Object> doc = TestSupport.conformance();
        String fixture = TestSupport.fixture();
        List<Map<String, Object>> cases = (List<Map<String, Object>>) doc.get("cases");
        List<DynamicNode> nodes = new ArrayList<>();
        for (Map<String, Object> c : cases) {
            String name = c.get("group") + "/" + c.get("name");
            nodes.add(DynamicTest.dynamicTest(name, () -> runCase(c, fixture)));
        }
        nodes.add(DynamicTest.dynamicTest("every expected reason code observed via typed API",
                () -> {
                    assertEquals(cases.size() + 1, nodes.size());
                    Set<String> wireOnly = new TreeSet<>(EXPECTED_ERRORS);
                    wireOnly.removeAll(OBSERVED_TYPED_ERRORS);
                    // Only codes that need invoke (missing required arg, stubs) may be absent.
                    assertEquals(Set.of(), without(wireOnly, "MISSING_FIELD", "NOT_IMPLEMENTED"));
                    if (typedSteps == 0 || wireSteps == 0) {
                        fail("typed " + typedSteps + " / wire " + wireSteps + " steps");
                    }
                }));
        return nodes.stream();
    }

    private static Set<String> without(Set<String> s, String... drop) {
        Set<String> out = new TreeSet<>(s);
        out.removeAll(List.of(drop));
        return out;
    }

    @SuppressWarnings("unchecked")
    private static void runCase(Map<String, Object> c, String fixture) {
        String config = fixture;
        int i = 0;
        for (Map<String, Object> step : (List<Map<String, Object>>) c.get("steps")) {
            try {
                config = runStep(step, config);
            } catch (AssertionError e) {
                throw new AssertionError("step " + i + " (" + step.get("fn") + "): "
                        + e.getMessage(), e);
            }
            i++;
        }
    }

    /** One step; returns the config for the next step. */
    @SuppressWarnings("unchecked")
    private static String runStep(Map<String, Object> step, String config) {
        String fn = (String) step.get("fn");
        Map<String, Object> args = (Map<String, Object>) step.get("args");
        Map<String, Object> expect = (Map<String, Object>) step.get("expect");
        String input = step.containsKey("config_literal") ? (String) step.get("config_literal")
                : config;
        String wantError = (String) expect.get("error");
        if (wantError != null) {
            synchronized (EXPECTED_ERRORS) {
                EXPECTED_ERRORS.add(wantError);
            }
        }
        boolean typed = !Boolean.TRUE.equals(step.get("wire_only"));
        String[] out;
        try {
            out = typed ? TypedDispatch.call(fn, input, args) : null;
            if (out == null) {
                typed = false;
                out = NativeBridge.invoke(fn, input, Json.write(args));
            }
        } catch (SzConfigToolException e) {
            count(typed);
            if (wantError == null) {
                fail("failed " + e.getReasonCode() + ": " + e.getMessage());
            }
            assertEquals(wantError, e.getReasonCode(), e.getMessage());
            assertEquals(SzConfigToolErrorKind.valueOf(wantError), e.getKind());
            if (typed) {
                synchronized (EXPECTED_ERRORS) {
                    OBSERVED_TYPED_ERRORS.add(wantError);
                }
            }
            return config;
        }
        count(typed);
        if (wantError != null) {
            fail("succeeded (" + out[0] + ") but expected " + wantError);
        }
        checkSuccess(expect, out);
        return out[1] != null ? out[1] : config;
    }

    private static synchronized void count(boolean typed) {
        if (typed) {
            typedSteps++;
        } else {
            wireSteps++;
        }
    }

    @Test
    void arrayChecksFailOnNonArrayResults() {
        // An object/int result must never satisfy len/contains/excludes vacuously.
        for (String expect : List.of("{\"len\":0}", "{\"excludes\":[{\"a\":1}]}",
                "{\"contains\":[{\"a\":1}]}")) {
            @SuppressWarnings("unchecked")
            Map<String, Object> e = (Map<String, Object>) Json.parse(expect);
            assertThrows(AssertionError.class,
                    () -> checkSuccess(e, new String[] {"json", null, "{\"a\":1}"}), expect);
            assertThrows(AssertionError.class,
                    () -> checkSuccess(e, new String[] {"int", null, "3"}), expect);
        }
        @SuppressWarnings("unchecked")
        Map<String, Object> ok = (Map<String, Object>) Json.parse("{\"len\":1,\"excludes\":[{\"a\":2}]}");
        assertDoesNotThrow(() -> checkSuccess(ok, new String[] {"json", null, "[{\"a\":1}]"}));
    }

    static void checkSuccess(Map<String, Object> expect, String[] out) {
        if (expect.containsKey("kind")) {
            assertEquals(expect.get("kind"), out[0], "kind");
        }
        Object result = out[2] == null ? null : Json.parse(out[2]);
        if (expect.containsKey("result") && !subset(expect.get("result"), result)) {
            fail("result " + out[2] + " does not contain " + Json.write(expect.get("result")));
        }
        boolean inspectsArray = expect.containsKey("len") || expect.containsKey("contains")
                || expect.containsKey("excludes");
        if (!inspectsArray) {
            return;
        }
        if (!(result instanceof List<?> items)) {
            fail("len/contains/excludes need an array result, got " + out[2]);
            return;
        }
        if (expect.containsKey("len")) {
            assertEquals(((Long) expect.get("len")).intValue(), items.size(), "len");
        }
        for (Object want : listOf(expect.get("contains"))) {
            if (items.stream().noneMatch(item -> subset(want, item))) {
                fail("no element matches " + Json.write(want));
            }
        }
        for (Object unwanted : listOf(expect.get("excludes"))) {
            if (items.stream().anyMatch(item -> subset(unwanted, item))) {
                fail("an element matches excluded " + Json.write(unwanted));
            }
        }
    }

    private static List<?> listOf(Object v) {
        return v instanceof List<?> l ? l : List.of();
    }

    /** {@code expected} is a subset of {@code actual} (schema.md "Subset match"). */
    static boolean subset(Object expected, Object actual) {
        if (expected instanceof Map<?, ?> e) {
            return actual instanceof Map<?, ?> a && e.entrySet().stream()
                    .allMatch(en -> a.containsKey(en.getKey())
                            && subset(en.getValue(), a.get(en.getKey())));
        }
        if (expected instanceof List<?> e) {
            if (!(actual instanceof List<?> a) || a.size() != e.size()) {
                return false;
            }
            for (int i = 0; i < e.size(); i++) {
                if (!subset(e.get(i), a.get(i))) {
                    return false;
                }
            }
            return true;
        }
        if (expected instanceof Number en && actual instanceof Number an) {
            return new BigDecimal(en.toString()).compareTo(new BigDecimal(an.toString())) == 0;
        }
        return Objects.equals(expected, actual);
    }
}
