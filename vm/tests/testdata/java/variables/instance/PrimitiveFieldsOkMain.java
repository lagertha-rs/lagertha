package variables.instance.primitive_fields;

public class PrimitiveFieldsOkMain {
    public static void main(String[] args) {
        PrimitiveFieldHolder holder = new PrimitiveFieldHolder();
        
        // Test initialized values
        assert holder.boolField == true : "prim.bool.init";
        assert holder.byteField == 42 : "prim.byte.init";
        assert holder.charField == 'X' : "prim.char.init";
        assert holder.shortField == 1000 : "prim.short.init";
        assert holder.intField == 123456 : "prim.int.init";
        assert holder.longField == 9876543210L : "prim.long.init";
        assert holder.floatField == 3.14f : "prim.float.init";
        assert holder.doubleField == 2.718281828 : "prim.double.init";

        // Test modification
        holder.boolField = false;
        assert holder.boolField == false : "prim.bool.mod";

        holder.byteField = -128;
        assert holder.byteField == -128 : "prim.byte.mod";

        holder.charField = '\u0000';
        assert holder.charField == 0 : "prim.char.mod";

        holder.shortField = Short.MIN_VALUE;
        assert holder.shortField == -32768 : "prim.short.mod";

        holder.intField = Integer.MAX_VALUE;
        assert holder.intField == 2147483647 : "prim.int.mod";

        holder.longField = Long.MIN_VALUE;
        assert holder.longField == -9223372036854775808L : "prim.long.mod";

        holder.floatField = -0.0f;
        assert holder.floatField == -0.0f : "prim.float.mod";

        holder.doubleField = 0.0;
        assert holder.doubleField == 0.0 : "prim.double.mod";

        System.out.println("Primitive fields test passed.");
    }
}

class PrimitiveFieldHolder {
    boolean boolField = true;
    byte byteField = 42;
    char charField = 'X';
    short shortField = 1000;
    int intField = 123456;
    long longField = 9876543210L;
    float floatField = 3.14f;
    double doubleField = 2.718281828;
}
