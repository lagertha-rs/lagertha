package variables.array_components.initializer.basic;

public class ArrayInitializerOkMain {
    public static void main(String[] args) {
        // Primitive array initializer
        int[] intArray = {1, 2, 3, 4, 5};
        assert intArray.length == 5 : "int array length";
        assert intArray[0] == 1 : "int element 0";
        assert intArray[4] == 5 : "int element 4";
        
        // Object array initializer
        String[] strArray = {"hello", "world"};
        assert strArray.length == 2 : "String array length";
        assert strArray[0].equals("hello") : "String element 0";
        assert strArray[1].equals("world") : "String element 1";
        
        // Multi-dimensional array initializer
        int[][] matrix = {{1, 2}, {3, 4}, {5, 6}};
        assert matrix.length == 3 : "2D array rows";
        assert matrix[0].length == 2 : "2D array cols";
        assert matrix[0][0] == 1 : "2D element 0,0";
        assert matrix[2][1] == 6 : "2D element 2,1";
        
        // Jagged array initializer
        int[][] jagged = {{1}, {2, 3}, {4, 5, 6}};
        assert jagged.length == 3 : "jagged rows";
        assert jagged[0].length == 1 : "jagged row 0 length";
        assert jagged[1].length == 2 : "jagged row 1 length";
        assert jagged[2].length == 3 : "jagged row 2 length";
        
        // Array initializer with expressions
        int x = 10;
        int[] exprArray = {x, x + 1, x * 2};
        assert exprArray[0] == 10 : "expression element 0";
        assert exprArray[1] == 11 : "expression element 1";
        assert exprArray[2] == 20 : "expression element 2";
        
        // Empty array initializer
        int[] empty = {};
        assert empty.length == 0 : "empty array";
        
        // Array initializer as argument
        assert sum(new int[]{1, 2, 3}) == 6 : "anonymous array initializer";
        
        // Default values in array initializer (only for omitted elements)
        // Not applicable; all elements must be provided.
        
        System.out.println("Array initializer tests passed.");
    }
    
    static int sum(int[] arr) {
        int total = 0;
        for (int val : arr) {
            total += val;
        }
        return total;
    }
}