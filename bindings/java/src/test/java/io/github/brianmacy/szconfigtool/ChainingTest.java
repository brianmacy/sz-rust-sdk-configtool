package io.github.brianmacy.szconfigtool;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.lang.reflect.Method;
import java.lang.reflect.Modifier;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

/**
 * Every config-changing method returns the new config text (issue #75): a
 * {@code config_and_json} function's primary returns only the config; its
 * companion {@code <name>Result} (same parameters) returns the record JSON
 * text, or the named-fields record for {@code tuple_names}.
 */
class ChainingTest {
    private static final String TEMPLATE = TestSupport.fixture();

    private static List<Method> methods(String name) {
        List<Method> out = new ArrayList<>();
        for (Method m : SzConfigTool.class.getDeclaredMethods()) {
            if (m.getName().equals(name) && Modifier.isPublic(m.getModifiers())) {
                out.add(m);
            }
        }
        return out;
    }

    private static String recordName(String snake) {
        String camel = TypedApiTest.camel(snake);
        return Character.toUpperCase(camel.charAt(0)) + camel.substring(1) + "Record";
    }

    @Test
    void configChangingPrimariesReturnStringAndPairsHaveCompanions() {
        int paired = 0;
        for (Map<String, Object> f : TestSupport.manifestFunctions()) {
            String returns = (String) f.get("returns");
            String name = (String) f.get("name");
            String java = TypedApiTest.camel(name);
            List<Method> companions = methods(java + "Result");
            if ("not_implemented".equals(f.get("status"))) {
                continue;
            }
            if (!"config_and_json".equals(returns)) {
                assertTrue(companions.isEmpty(), "no companion expected: " + java);
            }
            if (!"config".equals(returns) && !"config_and_json".equals(returns)) {
                continue;
            }
            List<Method> primaries = methods(java);
            assertFalse(primaries.isEmpty(), java);
            for (Method m : primaries) {
                assertEquals(String.class, m.getReturnType(), java);
            }
            if (!"config_and_json".equals(returns)) {
                continue;
            }
            paired++;
            assertEquals(primaries.size(), companions.size(), java + "Result overloads");
            String want = f.containsKey("tuple_names") ? recordName(name) : "String";
            for (Method p : primaries) {
                Method c = assertCompanion(java, p.getParameterTypes());
                assertEquals(want, c.getReturnType().getSimpleName(), java + "Result");
            }
        }
        assertTrue(paired > 0, "no config_and_json functions");
    }

    private static Method assertCompanion(String java, Class<?>[] params) {
        try {
            return SzConfigTool.class.getMethod(java + "Result", params);
        } catch (NoSuchMethodException e) {
            throw new AssertionError(java + "Result" + Arrays.toString(params), e);
        }
    }

    @Test
    void configAndJsonTypeIsGone() {
        assertThrows(ClassNotFoundException.class,
                () -> Class.forName("io.github.brianmacy.szconfigtool.ConfigAndJson"));
    }

    @Test
    void chainingSevenConfigChangingFunctions() throws Exception {
        String cfg = TEMPLATE;
        cfg = SzConfigTool.addElement(cfg, "DEMO_EL",
                new SzConfigTool.AddElementOptions().dataType("string"));
        cfg = SzConfigTool.addFeature(cfg, "DEMO_FEAT", "[\"DEMO_EL\"]");
        cfg = SzConfigTool.addAttribute(cfg, "DEMO_ATTR", "DEMO_FEAT", "DEMO_EL", "OTHER");
        cfg = SzConfigTool.addFragment(cfg,
                "{\"ERFRAG_CODE\":\"DEMO_FRAG\",\"ERFRAG_SOURCE\":\"./FRAGMENT[./SAME_NAME>0]\"}");
        cfg = SzConfigTool.addComparisonCall(cfg, "DEMO_FEAT", "EXACT_COMP", List.of("DEMO_EL"));
        cfg = SzConfigTool.addComparisonFunction(cfg, "DEMO_COMP");
        cfg = SzConfigTool.addDataSource(cfg, "DEMO_DS");
        Map<?, ?> attr = (Map<?, ?>) Json.parse(SzConfigTool.getAttribute(cfg, "DEMO_ATTR"));
        assertEquals("DEMO_ATTR", attr.get("ATTR_CODE"));
        assertTrue(SzConfigTool.getFragment(cfg, "DEMO_FRAG").contains("DEMO_FRAG"));
        assertTrue(SzConfigTool.getComparisonFunction(cfg, "DEMO_COMP").contains("DEMO_COMP"));
        assertTrue(SzConfigTool.listDataSources(cfg).contains("DEMO_DS"));
    }

    @Test
    void companionReturnsTheRowOfTheSameOperation() throws Exception {
        String row = SzConfigTool.addAttributeResult(TEMPLATE, "X_ATTR", "NAME", "FULL_NAME",
                "OTHER");
        String[] wire = NativeBridge.invoke("add_attribute", TEMPLATE,
                "{\"attribute\":\"X_ATTR\",\"feature\":\"NAME\",\"element\":\"FULL_NAME\","
                        + "\"class\":\"OTHER\"}");
        assertEquals(wire[2], row);
        assertEquals(wire[1],
                SzConfigTool.addAttribute(TEMPLATE, "X_ATTR", "NAME", "FULL_NAME", "OTHER"));
    }

    @Test
    void tupleNamesCompanionIsTheNamedRecordOnly() throws Exception {
        SzConfigTool.SetGenericPlanRecord created =
                SzConfigTool.setGenericPlanResult(TEMPLATE, "JAVA_PLAN", "Java plan");
        assertEquals(2, SzConfigTool.SetGenericPlanRecord.class.getRecordComponents().length);
        assertEquals("3", created.planId());
        assertEquals("true", created.wasCreated());
        String cfg = SzConfigTool.setGenericPlan(TEMPLATE, "JAVA_PLAN", "Java plan");
        assertTrue(cfg.contains("\"JAVA_PLAN\""));
        SzConfigTool.SetGenericPlanRecord updated =
                SzConfigTool.setGenericPlanResult(cfg, "INGEST", "Renamed");
        assertEquals("1", updated.planId());
        assertEquals("false", updated.wasCreated());
    }
}
