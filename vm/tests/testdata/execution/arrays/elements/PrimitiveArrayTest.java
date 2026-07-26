// @test feature = "execution.arrays.primitive-elements"
// @test description = "Verifies loads and stores for every primitive array component kind."
// @test category = "success"

package execution.arrays.elements;

public class PrimitiveArrayTest {
    public static void main(String[] args) {
        int index = runtime(1);

        boolean[] booleans = new boolean[2];
        booleans[index] = true;
        assert !booleans[0] : "primitive.boolean.false";
        assert booleans[index] : "primitive.boolean.true";

        byte[] bytes = new byte[2];
        bytes[index] = (byte) -128;
        assert bytes[index] == -128 : "primitive.byte.signed";

        char[] chars = new char[2];
        chars[index] = Character.MAX_VALUE;
        assert chars[index] == 65535 : "primitive.char.unsigned";

        short[] shorts = new short[2];
        shorts[index] = Short.MIN_VALUE;
        assert shorts[index] == -32768 : "primitive.short.signed";

        int[] integers = new int[2];
        integers[index] = Integer.MIN_VALUE;
        assert integers[index] == Integer.MIN_VALUE : "primitive.int";

        long[] longs = new long[2];
        longs[index] = Long.MIN_VALUE;
        assert longs[index] == Long.MIN_VALUE : "primitive.long";

        float[] floats = new float[2];
        floats[index] = -12.5f;
        assert floats[index] == -12.5f : "primitive.float";

        double[] doubles = new double[2];
        doubles[index] = 123.25d;
        assert doubles[index] == 123.25d : "primitive.double";

        int[] initialized = { runtime(4), 5, 6 };
        assert initialized[0] == 4 : "primitive.initializer";
        assert initialized[initialized[0] - 2] == 6 : "primitive.dynamic.index";
    }

    static int runtime(int value) {
        return value;
    }
}
