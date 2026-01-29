package variables.array_components.multidimensional.row_reassign;

public class RowReassignOkMain {
    public static void main(String[] args) {
        int[][] arr = new int[2][3];
        arr[0][0] = 1;
        arr[0][1] = 2;
        arr[0][2] = 3;
        
        // Create new sub-array
        int[] newRow = new int[3];
        newRow[0] = 10;
        newRow[1] = 20;
        newRow[2] = 30;
        
        // Replace first row
        arr[0] = newRow;
        assert arr[0][0] == 10 : "replaced.0";
        assert arr[0][1] == 20 : "replaced.1";
        assert arr[0][2] == 30 : "replaced.2";
        
        // They share the same reference
        newRow[0] = 100;
        assert arr[0][0] == 100 : "shared.ref";
        
        System.out.println("Row reassignment test passed.");
    }
}
