package variables.array_components.multidimensional.int_2d;

public class Int2dOkMain {
    public static void main(String[] args) {
        int[][] matrix = new int[3][4];
        
        assert matrix.length == 3 : "rows";
        assert matrix[0].length == 4 : "cols.0";
        assert matrix[1].length == 4 : "cols.1";
        assert matrix[2].length == 4 : "cols.2";
        
        // Write corners
        matrix[0][0] = 1;
        matrix[0][3] = 4;
        matrix[2][0] = 7;
        matrix[2][3] = 9;
        
        assert matrix[0][0] == 1 : "corner.00";
        assert matrix[0][3] == 4 : "corner.03";
        assert matrix[2][0] == 7 : "corner.20";
        assert matrix[2][3] == 9 : "corner.23";
        
        // Defaults are 0
        assert matrix[1][1] == 0 : "default";
        
        System.out.println("2D int array test passed.");
    }
}
