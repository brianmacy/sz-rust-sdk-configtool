package io.github.brianmacy.szconfigtool;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.Arrays;
import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;

/** Native construction of {@link SzConfigToolException} for each reachable code. */
class ErrorMappingTest {
    private static final String TEMPLATE = TestSupport.fixture();

    private static SzConfigToolException fails(String code, Executable call) {
        SzConfigToolException e = assertThrows(SzConfigToolException.class, call);
        assertEquals(code, e.getReasonCode(), e.getMessage());
        assertEquals(SzConfigToolErrorKind.valueOf(code), e.getKind());
        assertEquals(e.getReasonCode(), e.getKind().name());
        assertFalse(e.getMessage() == null || e.getMessage().isBlank());
        return e;
    }

    @Test
    void enumMatchesManifestReasonCodes() {
        assertEquals(TestSupport.reasonCodes(),
                Arrays.stream(SzConfigToolErrorKind.values()).map(Enum::name).toList());
        assertEquals(SzConfigToolErrorKind.INTERNAL, SzConfigToolErrorKind.fromCode("NOPE"));
        assertEquals(SzConfigToolErrorKind.INTERNAL, SzConfigToolErrorKind.fromCode(null));
        SzConfigToolException e = new SzConfigToolException("WHAT", "WHAT", "m", null);
        assertEquals(SzConfigToolErrorKind.INTERNAL, e.getKind());
        assertEquals("WHAT", e.getReasonCode());
    }

    @Test
    void libraryReasonCodes() {
        fails("JSON_PARSE", () -> SzConfigTool.listDataSources("not json"));
        fails("NOT_FOUND", () -> SzConfigTool.getDataSource(TEMPLATE, "NO_SUCH_DS"));
        fails("ALREADY_EXISTS", () -> SzConfigTool.addDataSource(TEMPLATE, "TEST"));
        fails("MISSING_SECTION", () -> SzConfigTool.listDataSources("{\"G2_CONFIG\":{}}"));
        fails("INVALID_STRUCTURE", () -> SzConfigTool.addSearchProfile("{\"G2_CONFIG\": "
                + "{\"CFG_GPLAN\": [{\"GPLAN_ID\": 2, \"GPLAN_CODE\": \"SEARCH\"}], "
                + "\"CFG_SPROFILE\": {}}}", "P2", "SEARCH"));
        fails("INVALID_CONFIG", () -> SzConfigTool.addFragment("{}",
                "{\"ERFRAG_CODE\": \"NEW_FRAG\", \"ERFRAG_SOURCE\": \"x\"}"));
        fails("ALREADY_PRESENT", () -> SzConfigTool.addComparisonCall(TEMPLATE, "gender",
                "EXACT_COMP", List.of("GENDER")));
        fails("NOT_ON_CALL",
                () -> SzConfigTool.deleteComparisonCallElement(TEMPLATE, "GENDER", "FULL_NAME"));
        fails("NOT_IN_FEATURE", () -> SzConfigTool.deleteComparisonCallElement(TEMPLATE, "NAME",
                "ADDR1", new SzConfigTool.DeleteComparisonCallElementOptions().elementFeature("NAME")));
        Map<String, Object> stub = stubStep();
        SzConfigToolException e = fails("NOT_IMPLEMENTED", () -> NativeBridge.invoke(
                (String) stub.get("fn"), TEMPLATE, Json.write(stub.get("args"))));
        assertNull(e.getDetails());
    }

    /** A conformance step calling a not_implemented function (stubs have no typed wrapper). */
    @SuppressWarnings("unchecked")
    private static Map<String, Object> stubStep() {
        for (Map<String, Object> c : (List<Map<String, Object>>) TestSupport.conformance().get("cases")) {
            for (Map<String, Object> step : (List<Map<String, Object>>) c.get("steps")) {
                Map<String, Object> expect = (Map<String, Object>) step.get("expect");
                if ("NOT_IMPLEMENTED".equals(expect.get("error")) && !step.containsKey("config_literal")) {
                    return step;
                }
            }
        }
        throw new AssertionError("no NOT_IMPLEMENTED conformance step");
    }

    @Test
    void validationErrorsCarryDetails() throws Exception {
        SzConfigToolException e = fails("VALIDATION_ERRORS",
                () -> SzConfigTool.setGenericThreshold(TEMPLATE, "INGEST", "NAME",
                        new SzConfigTool.SetGenericThresholdOptions().sendToRedo("sometimes")));
        Map<?, ?> details = (Map<?, ?>) Json.parse(e.getDetails());
        assertEquals("sz-configtool.validation-errors/v1", details.get("schema"));
        List<?> failures = (List<?>) details.get("failures");
        assertFalse(failures.isEmpty());
        Map<?, ?> first = (Map<?, ?>) failures.get(0);
        assertTrue(first.keySet().containsAll(List.of("field", "reasonCode", "offendingValue")),
                first.toString());
    }

    @Test
    void loneSurrogatesAreInvalidInputNotReplaced() throws Exception {
        // A lone UTF-16 surrogate has no UTF-8 form: it must be rejected, never
        // silently turned into U+FFFD (which would store a different code).
        fails("INVALID_INPUT", () -> SzConfigTool.addDataSource(TEMPLATE, "A\uD800"));
        fails("INVALID_INPUT", () -> SzConfigTool.addDataSource(TEMPLATE, "A\uDC00B"));
        fails("INVALID_INPUT", () -> NativeBridge.invoke("list_data_sources", TEMPLATE + "\uD83D", "{}"));
        fails("INVALID_INPUT", () -> NativeBridge.invoke("list\uD800", TEMPLATE, "{}"));
        // A valid surrogate PAIR (supplementary character) and NUL round-trip exactly.
        String added = SzConfigTool.addDataSource(TEMPLATE, "E\uD83D\uDE00\u0000X");
        String row = SzConfigTool.getDataSource(added, "E\uD83D\uDE00\u0000X");
        assertEquals("E\uD83D\uDE00\u0000X",
                ((Map<?, ?>) Json.parse(row)).get("DSRC_CODE"));
    }

    @Test
    void wireErrors() {
        fails("INVALID_INPUT", () -> NativeBridge.invoke("no_such_fn", TEMPLATE, "{}"));
        fails("INVALID_INPUT", () -> NativeBridge.invoke("list_data_sources", TEMPLATE, "[]"));
        fails("INVALID_INPUT", () -> NativeBridge.invoke("list_data_sources", TEMPLATE, "{x"));
        fails("INVALID_INPUT",
                () -> NativeBridge.invoke("add_data_source", TEMPLATE, "{\"code\":\"A\",\"cdoe\":1}"));
        fails("INVALID_INPUT", () -> NativeBridge.invoke("add_data_source", TEMPLATE, "{\"code\":1}"));
        fails("INVALID_INPUT",
                () -> NativeBridge.invoke("add_data_source", TEMPLATE, "{\"code\":\"A\",\"id\":null}"));
        fails("MISSING_FIELD", () -> NativeBridge.invoke("add_data_source", TEMPLATE, "{}"));
    }

    @Test
    void nullArgumentsAtTheNativeBoundary() throws Exception {
        fails("INVALID_INPUT", () -> NativeBridge.invoke(null, TEMPLATE, "{}"));
        fails("INVALID_INPUT", () -> NativeBridge.invoke("list_data_sources", null, "{}"));
        String[] out = NativeBridge.invoke("list_data_sources", TEMPLATE, null);
        assertEquals("json", out[0]);
        assertNull(out[1]);
    }

    @Test
    void exceptionIsCheckedAndIndependentOfSenzingSdk() {
        assertTrue(Exception.class.isAssignableFrom(SzConfigToolException.class));
        assertFalse(RuntimeException.class.isAssignableFrom(SzConfigToolException.class));
        assertEquals(Exception.class, SzConfigToolException.class.getSuperclass());
    }
}
