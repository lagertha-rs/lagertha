package variables.defaults.static_field_defaults;

public class StaticFieldDefaultsOkMain {
    public static void main(String[] args) {
        // boolean defaults to false
        assert StaticFieldDefaults.boolField == false : "static.bool.default";

        // byte defaults to 0
        assert StaticFieldDefaults.byteField == 0 : "static.byte.default";

        // short defaults to 0
        assert StaticFieldDefaults.shortField == 0 : "static.short.default";

        // char defaults to '\u0000' (0)
        assert StaticFieldDefaults.charField == '\u0000' : "static.char.default";
        assert StaticFieldDefaults.charField == 0 : "static.char.default.numeric";

        // int defaults to 0
        assert StaticFieldDefaults.intField == 0 : "static.int.default";

        // long defaults to 0L
        assert StaticFieldDefaults.longField == 0L : "static.long.default";

        // float defaults to 0.0f
        assert StaticFieldDefaults.floatField == 0.0f : "static.float.default";

        // double defaults to 0.0
        assert StaticFieldDefaults.doubleField == 0.0 : "static.double.default";

        // Object defaults to null
        assert StaticFieldDefaults.objectField == null : "static.object.default";

        // String defaults to null
        assert StaticFieldDefaults.stringField == null : "static.string.default";

        // Array defaults to null
        assert StaticFieldDefaults.arrayField == null : "static.array.default";

        // Custom class defaults to null
        assert StaticFieldDefaults.holderField == null : "static.holder.default";

        System.out.println("Static field defaults test passed.");
    }
}

class StaticFieldDefaults {
    static boolean boolField;
    static byte byteField;
    static short shortField;
    static char charField;
    static int intField;
    static long longField;
    static float floatField;
    static double doubleField;
    static Object objectField;
    static String stringField;
    static int[] arrayField;
    static Holder holderField;
}

class Holder {
    int value;
}
