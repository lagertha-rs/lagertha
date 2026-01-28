package variables.static_fields.primitive_statics;

public class PrimitiveStaticsOkMain {
    public static void main(String[] args) {
        // Test initialized values
        assert PrimitiveStatics.boolField == true : "static.bool.init";
        assert PrimitiveStatics.byteField == 42 : "static.byte.init";
        assert PrimitiveStatics.charField == 'S' : "static.char.init";
        assert PrimitiveStatics.shortField == 1000 : "static.short.init";
        assert PrimitiveStatics.intField == 123456 : "static.int.init";
        assert PrimitiveStatics.longField == 9876543210L : "static.long.init";
        assert PrimitiveStatics.floatField == 3.14f : "static.float.init";
        assert PrimitiveStatics.doubleField == 2.718281828 : "static.double.init";

        // Test modification
        PrimitiveStatics.boolField = false;
        assert PrimitiveStatics.boolField == false : "static.bool.mod";

        PrimitiveStatics.byteField = -128;
        assert PrimitiveStatics.byteField == -128 : "static.byte.mod";

        PrimitiveStatics.charField = '\uFFFF';
        assert PrimitiveStatics.charField == 65535 : "static.char.mod";

        PrimitiveStatics.shortField = Short.MAX_VALUE;
        assert PrimitiveStatics.shortField == 32767 : "static.short.mod";

        PrimitiveStatics.intField = Integer.MIN_VALUE;
        assert PrimitiveStatics.intField == -2147483648 : "static.int.mod";

        PrimitiveStatics.longField = Long.MAX_VALUE;
        assert PrimitiveStatics.longField == 9223372036854775807L : "static.long.mod";

        PrimitiveStatics.floatField = 0.0f;
        assert PrimitiveStatics.floatField == 0.0f : "static.float.mod";

        PrimitiveStatics.doubleField = -1.0;
        assert PrimitiveStatics.doubleField == -1.0 : "static.double.mod";

        System.out.println("Primitive static fields test passed.");
    }
}

class PrimitiveStatics {
    static boolean boolField = true;
    static byte byteField = 42;
    static char charField = 'S';
    static short shortField = 1000;
    static int intField = 123456;
    static long longField = 9876543210L;
    static float floatField = 3.14f;
    static double doubleField = 2.718281828;
}
