package variables.parameters.primitive_parameters;

public class PrimitiveParametersOkMain {
    public static void main(String[] args) {
        // boolean
        assert boolParam(true) == true : "prim.bool.true";
        assert boolParam(false) == false : "prim.bool.false";

        // byte
        assert byteParam((byte) 42) == 42 : "prim.byte";
        assert byteParam((byte) -128) == -128 : "prim.byte.min";
        assert byteParam((byte) 127) == 127 : "prim.byte.max";

        // char
        assert charParam('A') == 'A' : "prim.char";
        assert charParam('\u0000') == 0 : "prim.char.min";
        assert charParam('\uFFFF') == 65535 : "prim.char.max";

        // short
        assert shortParam((short) 1000) == 1000 : "prim.short";
        assert shortParam(Short.MIN_VALUE) == -32768 : "prim.short.min";
        assert shortParam(Short.MAX_VALUE) == 32767 : "prim.short.max";

        // int
        assert intParam(123456) == 123456 : "prim.int";
        assert intParam(Integer.MIN_VALUE) == Integer.MIN_VALUE : "prim.int.min";
        assert intParam(Integer.MAX_VALUE) == Integer.MAX_VALUE : "prim.int.max";

        // long
        assert longParam(9876543210L) == 9876543210L : "prim.long";
        assert longParam(Long.MIN_VALUE) == Long.MIN_VALUE : "prim.long.min";
        assert longParam(Long.MAX_VALUE) == Long.MAX_VALUE : "prim.long.max";

        // float
        assert floatParam(3.14f) == 3.14f : "prim.float";
        assert floatParam(0.0f) == 0.0f : "prim.float.zero";

        // double
        assert doubleParam(2.718281828) == 2.718281828 : "prim.double";
        assert doubleParam(0.0) == 0.0 : "prim.double.zero";

        System.out.println("Primitive parameters test passed.");
    }

    static boolean boolParam(boolean b) { return b; }
    static int byteParam(byte b) { return b; }
    static int charParam(char c) { return c; }
    static int shortParam(short s) { return s; }
    static int intParam(int i) { return i; }
    static long longParam(long l) { return l; }
    static float floatParam(float f) { return f; }
    static double doubleParam(double d) { return d; }
}
