package io.github.brianmacy.szconfigtool;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.function.Executable;

/**
 * The overload without an Options parameter is the empty-Options call, for
 * the functions whose conformance cases always pass some optional argument.
 */
class NoOptionsOverloadTest {
    private static final String T = TestSupport.fixture();

    @Test
    void addFeatureComparison() throws Exception {
        String out = SzConfigTool.addFeatureComparison(T, "EMAIL", "MAKE");
        assertNotEquals(T, out);
        assertEquals(SzConfigTool.addFeatureComparison(T, "EMAIL", "MAKE",
                new SzConfigTool.AddFeatureComparisonOptions()), out);
    }

    @Test
    void setFunctionsWithNothingToChange() throws Exception {
        assertEquals(SzConfigTool.setComparisonFunction(T, "STR_COMP",
                new SzConfigTool.SetComparisonFunctionOptions()),
                SzConfigTool.setComparisonFunction(T, "STR_COMP"));
        assertEquals(SzConfigTool.setExpressionFunction(T, "NAME_HASHER",
                new SzConfigTool.SetExpressionFunctionOptions()),
                SzConfigTool.setExpressionFunction(T, "NAME_HASHER"));
        assertEquals(SzConfigTool.setStandardizeFunction(T, "PARSE_NAME",
                new SzConfigTool.SetStandardizeFunctionOptions()),
                SzConfigTool.setStandardizeFunction(T, "PARSE_NAME"));
        assertEquals("STR_COMP", ((java.util.Map<?, ?>) Json.parse(
                SzConfigTool.setComparisonFunction(T, "STR_COMP").json())).get("CFUNC_CODE"));
    }

    @Test
    void setThresholdsWithNothingToChange() throws Exception {
        assertEquals(SzConfigTool.setComparisonThreshold(T, "GNR_COMP", "all", "GNR_FN",
                new SzConfigTool.SetComparisonThresholdOptions()),
                SzConfigTool.setComparisonThreshold(T, "GNR_COMP", "all", "GNR_FN"));
        assertEquals(SzConfigTool.setGenericThreshold(T, "INGEST", "FM",
                new SzConfigTool.SetGenericThresholdOptions()),
                SzConfigTool.setGenericThreshold(T, "INGEST", "FM"));
    }

    /**
     * Functions the library rejects without an optional argument
     * ({@code requires_options} in the manifest) get no options-less overload.
     */
    @Test
    void functionsThatNeedAnOptionalArgumentHaveNoOverloadWithoutOptions() {
        for (String name : new String[] {"addExpressionCall", "addStandardizeCall", "setFeature"}) {
            long withoutOptions = java.util.Arrays.stream(SzConfigTool.class.getMethods())
                    .filter(m -> m.getName().equals(name))
                    .filter(m -> !m.getParameterTypes()[m.getParameterCount() - 1].getSimpleName()
                            .endsWith("Options"))
                    .count();
            assertEquals(0, withoutOptions, name);
        }
    }

    private static void invalidInput(String message, Executable call) {
        SzConfigToolException e = assertThrows(SzConfigToolException.class, call);
        assertEquals("INVALID_INPUT", e.getReasonCode());
        assertEquals(message, e.getMessage());
    }

    /** For those functions a null Options is "none", which the library rejects. */
    @Test
    void nullOptionsOfARequiresOptionsFunctionIsRejected() {
        String either = "Invalid input: Either a feature or an element must be specified, but not both";
        invalidInput(either, () -> SzConfigTool.addExpressionCall(T, "EXPRESS_BOM",
                "[{\"element\":\"PHONE_NUM\",\"required\":\"Yes\",\"feature\":\"PHONE\"}]", "No", null));
        invalidInput(either, () -> SzConfigTool.addStandardizeCall(T, "PARSE_ID", null));
        invalidInput("Invalid input: No changes detected", () -> SzConfigTool.setFeature(T, "EMAIL", null));
    }
}
