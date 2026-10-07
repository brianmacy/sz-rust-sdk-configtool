package io.github.brianmacy.szconfigtool;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotEquals;

import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

/**
 * The configuration crosses JNI as an opaque string: what the library emits
 * reaches Java byte-for-byte, and what Java passes reaches the library unchanged.
 */
class OpacityTest {
    private static final String TEMPLATE = TestSupport.fixture();

    /** add + delete of a probe data source: the library's compact form of the input. */
    private static String roundTrip(String config) throws SzConfigToolException {
        String added = SzConfigTool.addDataSource(config, "OPACITY_PROBE");
        assertNotEquals(config, added);
        return SzConfigTool.deleteDataSource(added, "OPACITY_PROBE");
    }

    @Test
    void realTemplateRoundTripsByteExact() throws Exception {
        String expected = Json.write(Json.parse(TEMPLATE));
        String once = roundTrip(TEMPLATE);
        assertEquals(expected, once);
        assertEquals(once, roundTrip(once), "a returned config is passed back unchanged");
    }

    @Test
    void nonAsciiAndEscapesSurviveBothDirections() throws Exception {
        String note = "é 日本 😀 \"q\" \\ \t \u0001  ";
        StringBuilder lit = new StringBuilder();
        Json.quote(lit, note);
        String config = TEMPLATE.replaceFirst("\"G2_CONFIG\": \\{",
                "\"G2_CONFIG\": {\"ZZ_NOTE\": " + lit.toString().replace("\\", "\\\\") + ",");
        assertNotEquals(TEMPLATE, config);
        String out = roundTrip(config);
        assertEquals(Json.write(Json.parse(config)), out);
        Map<?, ?> g2 = (Map<?, ?>) ((Map<?, ?>) Json.parse(out)).get("G2_CONFIG");
        assertEquals(note, g2.get("ZZ_NOTE"));
    }

    @Test
    void supplementaryCharactersInArgsAndResults() throws Exception {
        String code = "DS_😀_日本";
        String config = SzConfigTool.addDataSource(TEMPLATE, code);
        List<?> rows = (List<?>) Json.parse(SzConfigTool.listDataSources(config));
        boolean found = rows.stream().map(r -> (Map<?, ?>) r)
                .anyMatch(r -> r.containsValue(code.toUpperCase(java.util.Locale.ROOT)));
        assertEquals(true, found, rows.toString());
    }
}
