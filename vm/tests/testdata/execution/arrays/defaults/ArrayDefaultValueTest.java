// @test feature = "execution.arrays.default-values"
// @test description = "Verifies default values for primitive, reference, and nested array components."
// @test category = "success"

package execution.arrays.defaults;

public class ArrayDefaultValueTest {
    public static void main(String[] args) {
        int index = runtime(1);
        assert !(new boolean[2])[index] : "defaults.boolean";
        assert (new byte[2])[index] == 0 : "defaults.byte";
        assert (new char[2])[index] == 0 : "defaults.char";
        assert (new short[2])[index] == 0 : "defaults.short";
        assert (new int[2])[index] == 0 : "defaults.int";
        assert (new long[2])[index] == 0L : "defaults.long";
        assert (new float[2])[index] == 0.0f : "defaults.float";
        assert (new double[2])[index] == 0.0d : "defaults.double";
        assert (new Object[2])[index] == null : "defaults.reference";

        int[][] partial = new int[2][];
        assert partial[0] == null : "defaults.unallocated.row";
        partial[0] = new int[2];
        partial[1] = new int[2];
        partial[0][index] = 9;
        assert partial[1][index] == 0 : "defaults.independent.row";
    }

    static int runtime(int value) {
        return value;
    }
}
