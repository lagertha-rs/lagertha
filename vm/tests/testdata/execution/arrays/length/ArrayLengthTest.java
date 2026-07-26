// @test feature = "execution.arrays.length"
// @test description = "Verifies primitive, reference, nested, empty, and null array length behavior."
// @test category = "success"

package execution.arrays.length;

public class ArrayLengthTest {
    public static void main(String[] args) {
        int length = runtime(3);
        assert new int[0].length == 0 : "length.empty";
        assert new int[length].length == 3 : "length.primitive";
        assert new Object[length - 1].length == 2 : "length.reference";

        int[][] jagged = new int[2][];
        jagged[0] = new int[length];
        jagged[1] = new int[length + 2];
        assert jagged.length == 2 : "length.outer";
        assert jagged[0].length == 3 : "length.first.row";
        assert jagged[1].length == 5 : "length.second.row";

        int[] nullable = null;
        try {
            int ignored = nullable.length;
            assert ignored < 0 : "length.null.missing.exception";
        } catch (NullPointerException expected) {
            assert nullable == null : "length.null";
        }
    }

    static int runtime(int value) {
        return value;
    }
}
