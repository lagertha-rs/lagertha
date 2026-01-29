package variables.array_components.multidimensional.primitives_2d;

public class Primitives2dOkMain {
    public static void main(String[] args) {
        // byte[][]
        byte[][] bytes = new byte[2][2];
        bytes[0][0] = 127;
        bytes[1][1] = -128;
        assert bytes[0][0] == 127 : "byte.max";
        assert bytes[1][1] == -128 : "byte.min";
        assert bytes[0][1] == 0 : "byte.default";

        // short[][]
        short[][] shorts = new short[2][2];
        shorts[0][0] = 32767;
        shorts[1][1] = -32768;
        assert shorts[0][0] == 32767 : "short.max";
        assert shorts[1][1] == -32768 : "short.min";

        // long[][]
        long[][] longs = new long[2][2];
        longs[0][0] = 9223372036854775807L;
        longs[1][1] = -9223372036854775808L;
        assert longs[0][0] == 9223372036854775807L : "long.max";
        assert longs[1][1] == -9223372036854775808L : "long.min";

        // char[][]
        char[][] chars = new char[2][2];
        chars[0][0] = 'A';
        chars[1][1] = '\u0000';
        assert chars[0][0] == 'A' : "char.A";
        assert chars[1][1] == '\u0000' : "char.null";

        // boolean[][]
        boolean[][] bools = new boolean[2][2];
        bools[0][0] = true;
        bools[1][1] = false;
        assert bools[0][0] == true : "bool.true";
        assert bools[0][1] == false : "bool.default";

        System.out.println("2D primitives array test passed.");
    }
}
