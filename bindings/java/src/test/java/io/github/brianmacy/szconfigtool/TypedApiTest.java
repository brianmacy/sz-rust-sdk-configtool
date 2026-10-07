package io.github.brianmacy.szconfigtool;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.lang.reflect.Method;
import java.lang.reflect.Modifier;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import org.junit.jupiter.api.Test;

/** Naming, overloads, required args and tri-state args of the generated API. */
class TypedApiTest {
    private static final String TEMPLATE = TestSupport.fixture();

    /** The contract's rule: split on '_', no acronym special-casing. */
    static String camel(String snake) {
        StringBuilder sb = new StringBuilder();
        for (String part : snake.split("_")) {
            if (part.isEmpty()) {
                continue;
            }
            sb.append(sb.length() == 0 ? part.toLowerCase(Locale.ROOT)
                    : Character.toUpperCase(part.charAt(0)) + part.substring(1));
        }
        return sb.toString();
    }

    private static List<Method> methods(String name) {
        List<Method> out = new ArrayList<>();
        for (Method m : SzConfigTool.class.getDeclaredMethods()) {
            if (m.getName().equals(name) && Modifier.isPublic(m.getModifiers())) {
                out.add(m);
            }
        }
        return out;
    }

    @Test
    @SuppressWarnings("unchecked")
    void everyImplementedFunctionHasCamelCaseOverloads() {
        int implemented = 0;
        for (Map<String, Object> f : TestSupport.manifestFunctions()) {
            String java = camel((String) f.get("name"));
            List<Method> ms = methods(java);
            if ("not_implemented".equals(f.get("status"))) {
                assertTrue(ms.isEmpty(), "stub must be skipped: " + java);
                continue;
            }
            implemented++;
            List<Map<String, Object>> args = (List<Map<String, Object>>) f.get("args");
            long positional = args.stream().filter(a -> !(Boolean) a.get("optional")
                    && !(Boolean) a.get("tristate") || (Boolean) a.get("required")).count();
            // requires_options: only the Options overload (no options-less forwarder).
            boolean twoForms = args.size() > positional
                    && !Boolean.TRUE.equals(f.get("requires_options"));
            // Each positional int_or_str arg doubles the overloads (long / String).
            long intOrStr = args.stream().filter(a -> "int_or_str".equals(a.get("type"))
                    && (!(Boolean) a.get("optional") || (Boolean) a.get("required"))).count();
            assertEquals((twoForms ? 2 : 1) << intOrStr, ms.size(), java);
            for (Method m : ms) {
                assertTrue(Modifier.isStatic(m.getModifiers()), java);
                assertEquals(String.class, m.getParameterTypes()[0], java);
                assertEquals(List.of(SzConfigToolException.class),
                        Arrays.asList(m.getExceptionTypes()), java);
                int expected = (int) positional + 1 + (m.getParameterCount() > positional + 1 ? 1 : 0);
                assertEquals(expected, m.getParameterCount(), java);
            }
        }
        assertTrue(implemented > 100, "implemented=" + implemented);
    }

    @Test
    void classIsFinalWithNoInstances() throws Exception {
        assertTrue(Modifier.isFinal(SzConfigTool.class.getModifiers()));
        assertEquals(0, SzConfigTool.class.getConstructors().length);
        assertFalse(SzConfigTool.class.getName().endsWith(".SzConfig"));
    }

    @Test
    void requiredOptionArgIsPositionalAndRejectsNull() throws Exception {
        // set_generic_threshold: plan, behavior are optional+required in the manifest.
        Method m = SzConfigTool.class.getMethod("setGenericThreshold", String.class,
                String.class, String.class);
        assertNotNull(m);
        assertThrows(NullPointerException.class,
                () -> SzConfigTool.setGenericThreshold(TEMPLATE, null, "NAME"));
        // On the wire, omitting it reaches the library as None -> MISSING_FIELD.
        SzConfigToolException e = assertThrows(SzConfigToolException.class,
                () -> NativeBridge.invoke("set_generic_threshold", TEMPLATE, "{\"behavior\":\"NAME\"}"));
        assertEquals("MISSING_FIELD", e.getReasonCode());
    }

    @Test
    void reservedWordArgIsRenamed() throws Exception {
        SzConfigTool.AddAttributeOptions o =
                new SzConfigTool.AddAttributeOptions().internal("yes").id(7777);
        String json = SzConfigTool.addAttributeResult(TEMPLATE, "JAVA_ATTR", "NAME", "FULL_NAME",
                "NAME", o);
        Map<?, ?> row = (Map<?, ?>) Json.parse(json);
        assertEquals(7777L, row.get("ATTR_ID"));
        assertEquals("Yes", row.get("INTERNAL"));
        String config = SzConfigTool.addAttribute(TEMPLATE, "JAVA_ATTR", "NAME", "FULL_NAME",
                "NAME", o);
        Map<?, ?> got = (Map<?, ?>) Json.parse(SzConfigTool.getAttribute(config, "JAVA_ATTR"));
        assertEquals("NAME", got.get("ATTR_CLASS"));
    }

    @Test
    void nullOptionsMeansNoOptionalArgs() throws Exception {
        String a = SzConfigTool.addDataSource(TEMPLATE, "NULLOPTS", null);
        String b = SzConfigTool.addDataSource(TEMPLATE, "NULLOPTS");
        assertEquals(a, b);
    }

    @Test
    void triStateLeaveClearSet() throws Exception {
        String set = SzConfigTool.setFragment(TEMPLATE, "SNAME_SSTAB",
                new SzConfigTool.SetFragmentOptions().description(FieldUpdate.set("java desc")));
        assertEquals("java desc", fragmentDesc(set));
        String left = SzConfigTool.setFragment(set, "SNAME_SSTAB",
                new SzConfigTool.SetFragmentOptions().description(FieldUpdate.leave()));
        assertEquals("java desc", fragmentDesc(left));
        String cleared = SzConfigTool.setFragment(left, "SNAME_SSTAB",
                new SzConfigTool.SetFragmentOptions().description(FieldUpdate.clear()));
        assertEquals(null, fragmentDesc(cleared));
        // A later leave() removes an earlier set/clear from the same builder.
        SzConfigTool.SetFragmentOptions o = new SzConfigTool.SetFragmentOptions()
                .description(FieldUpdate.clear()).description(FieldUpdate.leave());
        assertEquals("java desc", fragmentDesc(SzConfigTool.setFragment(set, "SNAME_SSTAB", o)));
    }

    private static String fragmentDesc(String config) throws SzConfigToolException {
        List<?> rows = (List<?>) Json.parse(SzConfigTool.getConfigSection(config, "CFG_ERFRAG",
                new SzConfigTool.GetConfigSectionOptions().filter("\"ERFRAG_ID\": 180")));
        assertEquals(1, rows.size());
        return (String) ((Map<?, ?>) rows.get(0)).get("ERFRAG_DESC");
    }

    @Test
    void tupleNamesBecomeRecordFields() throws Exception {
        // Fields are each the record member's JSON text (strings keep their quotes).
        SzConfigTool.VerifyCompatibilityVersionRecord r =
                SzConfigTool.verifyCompatibilityVersion(TEMPLATE, "11");
        assertEquals("\"11\"", r.currentVersion());
        assertEquals("true", r.matches());
        SzConfigTool.VerifyCompatibilityVersionRecord miss =
                SzConfigTool.verifyCompatibilityVersion(TEMPLATE, "99");
        assertEquals("\"11\"", miss.currentVersion());
        assertEquals("false", miss.matches());
    }

    @Test
    void intOrStrSelectorHasLongAndStringOverloads() throws Exception {
        assertNotNull(SzConfigTool.class.getMethod("getStandardizeCall", String.class, long.class));
        assertNotNull(SzConfigTool.class.getMethod("getStandardizeCall", String.class, String.class));
        // CFG_SFCALL 2 is the fixture's only standardize call on DOB.
        String byId = SzConfigTool.getStandardizeCall(TEMPLATE, 2L);
        String byFeature = SzConfigTool.getStandardizeCall(TEMPLATE, "dob");
        assertEquals(byId, byFeature);
        assertEquals(2L, ((Map<?, ?>) Json.parse(byId)).get("SFCALL_ID"));
        // The String overload is a feature code, never a numeric id.
        SzConfigToolException e = assertThrows(SzConfigToolException.class,
                () -> SzConfigTool.getStandardizeCall(TEMPLATE, "2"));
        assertEquals("NOT_FOUND", e.getReasonCode());
        SzConfigToolException missing = assertThrows(SzConfigToolException.class,
                () -> SzConfigTool.getStandardizeCall(TEMPLATE, 999_999L));
        assertEquals("NOT_FOUND", missing.getReasonCode());
    }

    @Test
    void intOrStrSelectorOnDelete() throws Exception {
        // Mirrors conformance: put FULL_NAME on comparison call 4 (GENDER), then
        // delete it by feature code (String) and GENDER by call id (long).
        String added = SzConfigTool.addComparisonCallElement(TEMPLATE, 4, 4, 2);
        String byFeature = SzConfigTool.deleteComparisonCallElement(added, "gender", "full_name");
        assertEquals(List.of("GENDER"), elementList(byFeature, 4L));
        String byId = SzConfigTool.deleteComparisonCallElement(byFeature, 4L, "GENDER");
        assertEquals(List.of(), elementList(byId, 4L));
    }

    private static Object elementList(String config, long id) throws SzConfigToolException {
        for (Object row : (List<?>) Json.parse(SzConfigTool.listComparisonCalls(config))) {
            Map<?, ?> m = (Map<?, ?>) row;
            if (Long.valueOf(id).equals(m.get("id"))) {
                return m.get("elementList");
            }
        }
        throw new AssertionError("no comparison call " + id);
    }

    @Test
    void jsonArgIsValidatedBeforeTheCall() {
        SzConfigTool.AddSearchProfileOptions o = new SzConfigTool.AddSearchProfileOptions();
        assertThrows(IllegalArgumentException.class, () -> o.elements("[1, \"id\": 5"));
        assertThrows(NullPointerException.class, () -> SzConfigTool.addDataSource(null, "X"));
        assertThrows(NullPointerException.class, () -> SzConfigTool.addDataSource(TEMPLATE, null));
    }
}
