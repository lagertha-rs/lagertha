package variables.local.primitive_types;

public class PrimitiveTypesOkMain {
    public static void main(String[] args) {
        // boolean
        boolean boolVar = true;
        assert boolVar == true : "bool.init.true";
        boolVar = false;
        assert boolVar == false : "bool.assign.false";

        // byte
        byte byteVar = 42;
        assert byteVar == 42 : "byte.init";
        byteVar = -128;
        assert byteVar == -128 : "byte.min";
        byteVar = 127;
        assert byteVar == 127 : "byte.max";

        // char
        char charVar = 'A';
        assert charVar == 'A' : "char.init";
        assert charVar == 65 : "char.numeric";
        charVar = '\u0000';
        assert charVar == 0 : "char.null";
        charVar = '\uFFFF';
        assert charVar == 65535 : "char.max";

        // short
        short shortVar = 1000;
        assert shortVar == 1000 : "short.init";
        shortVar = -32768;
        assert shortVar == Short.MIN_VALUE : "short.min";
        shortVar = 32767;
        assert shortVar == Short.MAX_VALUE : "short.max";

        // int
        int intVar = 123456;
        assert intVar == 123456 : "int.init";
        intVar = Integer.MIN_VALUE;
        assert intVar == -2147483648 : "int.min";
        intVar = Integer.MAX_VALUE;
        assert intVar == 2147483647 : "int.max";

        // long
        long longVar = 9876543210L;
        assert longVar == 9876543210L : "long.init";
        longVar = Long.MIN_VALUE;
        assert longVar == -9223372036854775808L : "long.min";
        longVar = Long.MAX_VALUE;
        assert longVar == 9223372036854775807L : "long.max";

        // float
        float floatVar = 3.14f;
        assert floatVar == 3.14f : "float.init";
        floatVar = 0.0f;
        assert floatVar == 0.0f : "float.zero";
        floatVar = -1.5f;
        assert floatVar == -1.5f : "float.neg";

        // double
        double doubleVar = 3.141592653589793;
        assert doubleVar == 3.141592653589793 : "double.init";
        doubleVar = 0.0;
        assert doubleVar == 0.0 : "double.zero";
        doubleVar = -2.718281828;
        assert doubleVar == -2.718281828 : "double.neg";

        System.out.println("All primitive type local variable tests passed.");
    }
}
