// @test feature = "execution.arrays.multidimensional"
// @test description = "Verifies rectangular primitive and reference arrays across multiple ranks."
// @test category = "success"

package execution.arrays.multidimensional;

public class RectangularArrayTest {
    public static void main(String[] args) {
        int rows = runtime(2);
        int columns = rows + 1;
        int[][] integers = new int[rows][columns];
        integers[rows - 1][columns - 1] = 23;
        assert integers.length == 2 : "rectangular.rows";
        assert integers[0].length == 3 : "rectangular.columns";
        assert integers[1][2] == 23 : "rectangular.int.corner";
        assert integers[0][1] == 0 : "rectangular.int.default";

        long[][][] cubes = new long[rows][1][columns];
        cubes[1][0][2] = Long.MIN_VALUE;
        assert cubes[1][0][2] == Long.MIN_VALUE : "rectangular.long.rank3";

        int[][][][] rankFour = new int[1][rows][1][columns];
        rankFour[0][1][0][2] = 41;
        assert rankFour[0][1][0][2] == 41 : "rectangular.int.rank4";

        Object marker = new Object();
        Object[][] references = new Object[rows][columns];
        references[1][2] = marker;
        assert references[1][2] == marker : "rectangular.reference";
        assert references[0][0] == null : "rectangular.reference.default";
    }

    static int runtime(int value) {
        return value;
    }
}
