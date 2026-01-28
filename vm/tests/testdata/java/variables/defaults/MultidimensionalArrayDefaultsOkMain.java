package variables.defaults.multidimensional_array_defaults;

public class MultidimensionalArrayDefaultsOkMain {
    public static void main(String[] args) {
        // 2D array with both dimensions specified
        int[][] matrix = new int[3][4];
        
        // All rows should be non-null (auto-created)
        assert matrix[0] != null : "2d.row0.notnull";
        assert matrix[1] != null : "2d.row1.notnull";
        assert matrix[2] != null : "2d.row2.notnull";
        
        // Row lengths
        assert matrix[0].length == 4 : "2d.row0.len";
        assert matrix[1].length == 4 : "2d.row1.len";
        assert matrix[2].length == 4 : "2d.row2.len";

        // All elements default to 0
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 4; j++) {
                assert matrix[i][j] == 0 : "2d.elem.default";
            }
        }

        // 2D array with only first dimension specified
        int[][] partialMatrix = new int[3][];
        assert partialMatrix[0] == null : "2d.partial.row0.null";
        assert partialMatrix[1] == null : "2d.partial.row1.null";
        assert partialMatrix[2] == null : "2d.partial.row2.null";

        // 3D array
        int[][][] cube = new int[2][3][4];
        assert cube[0] != null : "3d.dim0.notnull";
        assert cube[0][0] != null : "3d.dim1.notnull";
        assert cube[0][0][0] == 0 : "3d.elem.default";
        assert cube[1][2][3] == 0 : "3d.elem.default.last";

        // 2D boolean array
        boolean[][] boolMatrix = new boolean[2][2];
        assert boolMatrix[0][0] == false : "2d.bool.default.00";
        assert boolMatrix[1][1] == false : "2d.bool.default.11";

        // 2D object array
        Object[][] objMatrix = new Object[2][2];
        assert objMatrix[0][0] == null : "2d.obj.default.00";
        assert objMatrix[1][1] == null : "2d.obj.default.11";

        System.out.println("Multidimensional array defaults test passed.");
    }
}
