package io.github.brianmacy.szconfigtool;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.lang.reflect.Modifier;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import org.junit.jupiter.api.Test;

/**
 * Every setter of every generated {@code *Options} builder writes its manifest
 * argument under the manifest name with the right JSON rendering (the
 * conformance cases pass only some optional arguments).
 */
class OptionsSettersTest {
    /** Marks "argument omitted" (a LEAVE update). */
    private static final Object ABSENT = new Object();

    /** One setter input and the JSON value expected on the wire (or {@link #ABSENT}). */
    private record Sample(Object input, Object wire) {
    }

    private static String pascal(String snake) {
        String c = TypedApiTest.camel(snake);
        return Character.toUpperCase(c.charAt(0)) + c.substring(1);
    }

    private static List<Sample> samples(Class<?> param, Map<String, Object> arg) {
        boolean intArg = "int".equals(arg.get("type"));
        if (param == FieldUpdate.class) {
            Object v = intArg ? (Object) 7L : "v";
            return List.of(new Sample(FieldUpdate.set(v), v), new Sample(FieldUpdate.clear(), null),
                    new Sample(FieldUpdate.leave(), ABSENT));
        }
        if (param == long.class) {
            return List.of(new Sample(7L, 7L));
        }
        if (param == boolean.class) {
            return List.of(new Sample(true, true), new Sample(false, false));
        }
        if (param == List.class) {
            return List.of(new Sample(List.of("a", "b"), List.of("a", "b")));
        }
        assertEquals(String.class, param, String.valueOf(arg.get("name")));
        return "json".equals(arg.get("type"))
                ? List.of(new Sample("{\"k\":[1]}", Map.of("k", List.of(1L))))
                : List.of(new Sample("v", "v"));
    }

    private static boolean positional(Map<String, Object> a) {
        return !(Boolean) a.get("optional") && !(Boolean) a.get("tristate")
                || (Boolean) a.get("required");
    }

    private static List<Method> setters(Class<?> cls, String snake) {
        String id = TypedApiTest.camel(snake);
        List<Method> out = new ArrayList<>();
        for (Method m : cls.getDeclaredMethods()) {
            if (m.getName().equals(id) || m.getName().equals(id + "Value")) {
                out.add(m);
            }
        }
        return out;
    }

    @Test
    @SuppressWarnings("unchecked")
    void everySetterRendersItsArgument() throws Exception {
        Set<Method> tested = new HashSet<>();
        for (Map<String, Object> f : TestSupport.manifestFunctions()) {
            if ("not_implemented".equals(f.get("status"))) {
                continue;
            }
            for (Map<String, Object> a : (List<Map<String, Object>>) f.get("args")) {
                if (positional(a)) {
                    continue;
                }
                Class<?> cls = Class.forName(SzConfigTool.class.getName() + "$"
                        + pascal((String) f.get("name")) + "Options");
                List<Method> ms = setters(cls, (String) a.get("name"));
                assertFalse(ms.isEmpty(), cls.getSimpleName() + " lacks " + a.get("name"));
                for (Method m : ms) {
                    for (Sample s : samples(m.getParameterTypes()[0], a)) {
                        assertWire(cls, m, (String) a.get("name"), s);
                    }
                    tested.add(m);
                }
            }
        }
        assertEquals(allOptionSetters(), tested);
    }

    private static void assertWire(Class<?> cls, Method m, String name, Sample s) throws Exception {
        Object builder = cls.getDeclaredConstructor().newInstance();
        assertSame(builder, m.invoke(builder, s.input()), m.toString());
        Field wire = cls.getDeclaredField("wire");
        Object parsed = Json.parse(((Args) wire.get(builder)).toJson());
        Map<String, Object> expected = s.wire() == ABSENT ? Map.of()
                : Collections.singletonMap(name, s.wire());
        assertEquals(expected, parsed, m + " with " + s.input());
    }

    private static Set<Method> allOptionSetters() {
        Set<Method> all = new HashSet<>();
        for (Class<?> c : SzConfigTool.class.getDeclaredClasses()) {
            if (!c.getSimpleName().endsWith("Options")) {
                continue;
            }
            for (Method m : c.getDeclaredMethods()) {
                if (Modifier.isPublic(m.getModifiers())) {
                    all.add(m);
                }
            }
        }
        assertTrue(all.size() > 50, "setters=" + all.size());
        return all;
    }
}
