package variables.array_components.multidimensional.nested_access;

public class NestedAccessOkMain {
    public static void main(String[] args) {
        int[][] arr = new int[3][3];
        
        // Set diagonal
        arr[0][0] = 1;
        arr[1][1] = 2;
        arr[2][2] = 3;
        
        // Sum diagonal
        int sum = arr[0][0] + arr[1][1] + arr[2][2];
        assert sum == 6 : "diagonal.sum";
        
        // Access via intermediate reference
        int[] row = arr[1];
        assert row[1] == 2 : "row.access";
        
        // Modify via row reference
        row[0] = 100;
        assert arr[1][0] == 100 : "row.modified";
        
        // Row references are same object
        int[] row2 = arr[1];
        assert row == row2 : "row.identity";
        
        System.out.println("Nested access test passed.");
    }
}
