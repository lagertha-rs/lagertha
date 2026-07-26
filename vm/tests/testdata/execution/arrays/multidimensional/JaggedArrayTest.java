// @test feature = "execution.arrays.multidimensional"
// @test description = "Verifies independently allocated, empty, reassigned, aliased, and partial nested rows."
// @test category = "success"

package execution.arrays.multidimensional;

public class JaggedArrayTest {
    public static void main(String[] args) {
        int rowCount = runtime(4);
        int[][] jagged = new int[rowCount][];
        assert jagged[0] == null : "jagged.null.row";

        jagged[0] = new int[0];
        jagged[1] = new int[1];
        jagged[2] = new int[3];
        jagged[3] = jagged[2];
        jagged[2][2] = 17;
        assert jagged[0].length == 0 : "jagged.empty.row";
        assert jagged[1].length == 1 : "jagged.short.row";
        assert jagged[3][2] == 17 : "jagged.aliased.row";

        jagged[1] = new int[5];
        assert jagged[1].length == 5 : "jagged.reassigned.row";
        assert jagged[1][4] == 0 : "jagged.reassigned.default";

        int[][] triangle = new int[rowCount][];
        for (int row = 0; row < triangle.length; row++) {
            triangle[row] = new int[row + 1];
            triangle[row][row] = row + 10;
        }
        assert triangle[3].length == 4 : "jagged.triangle.length";
        assert triangle[3][3] == 13 : "jagged.triangle.value";

        int[][][] partial = new int[2][][];
        partial[0] = new int[1][];
        partial[0][0] = new int[] { 29 };
        assert partial[0][0][0] == 29 : "jagged.rank3.value";
        assert partial[1] == null : "jagged.rank3.partial";
    }

    static int runtime(int value) {
        return value;
    }
}
