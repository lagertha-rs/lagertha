package variables.defaults.array_element_defaults;

public class ArrayElementDefaultsOkMain {
    public static void main(String[] args) {
        // boolean array elements default to false
        boolean[] boolArr = new boolean[5];
        for (int i = 0; i < 5; i++) {
            assert boolArr[i] == false : "arr.bool.default";
        }

        // byte array elements default to 0
        byte[] byteArr = new byte[5];
        for (int i = 0; i < 5; i++) {
            assert byteArr[i] == 0 : "arr.byte.default";
        }

        // short array elements default to 0
        short[] shortArr = new short[5];
        for (int i = 0; i < 5; i++) {
            assert shortArr[i] == 0 : "arr.short.default";
        }

        // char array elements default to '\u0000'
        char[] charArr = new char[5];
        for (int i = 0; i < 5; i++) {
            assert charArr[i] == '\u0000' : "arr.char.default";
            assert charArr[i] == 0 : "arr.char.default.numeric";
        }

        // int array elements default to 0
        int[] intArr = new int[5];
        for (int i = 0; i < 5; i++) {
            assert intArr[i] == 0 : "arr.int.default";
        }

        // long array elements default to 0L
        long[] longArr = new long[5];
        for (int i = 0; i < 5; i++) {
            assert longArr[i] == 0L : "arr.long.default";
        }

        // float array elements default to 0.0f
        float[] floatArr = new float[5];
        for (int i = 0; i < 5; i++) {
            assert floatArr[i] == 0.0f : "arr.float.default";
        }

        // double array elements default to 0.0
        double[] doubleArr = new double[5];
        for (int i = 0; i < 5; i++) {
            assert doubleArr[i] == 0.0 : "arr.double.default";
        }

        // Large arrays also have defaults
        int[] largeArr = new int[1000];
        assert largeArr[0] == 0 : "arr.large.first";
        assert largeArr[500] == 0 : "arr.large.mid";
        assert largeArr[999] == 0 : "arr.large.last";

        // Zero-length array (edge case)
        int[] emptyArr = new int[0];
        assert emptyArr.length == 0 : "arr.empty.len";

        System.out.println("Array element defaults test passed.");
    }
}
