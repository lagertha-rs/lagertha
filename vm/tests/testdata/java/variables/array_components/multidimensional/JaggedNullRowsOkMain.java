package variables.array_components.multidimensional.jagged_null_rows;

public class JaggedNullRowsOkMain {
    public static void main(String[] args) {
        // When only outer dimension specified, sub-arrays are null
        int[][] jagged = new int[3][];
        
        assert jagged[0] == null : "null.row0";
        assert jagged[1] == null : "null.row1";
        assert jagged[2] == null : "null.row2";
        
        // Initialize one row
        jagged[0] = new int[5];
        assert jagged[0] != null : "assigned.row0";
        assert jagged[0].length == 5 : "assigned.row0.len";
        assert jagged[1] == null : "still.null.row1";
        
        // Elements in initialized row default to 0
        assert jagged[0][0] == 0 : "default.0";
        assert jagged[0][4] == 0 : "default.4";
        
        System.out.println("Jagged null rows test passed.");
    }
}
