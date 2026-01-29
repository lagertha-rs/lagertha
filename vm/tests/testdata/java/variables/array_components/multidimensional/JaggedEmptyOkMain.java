package variables.array_components.multidimensional.jagged_empty;

public class JaggedEmptyOkMain {
    public static void main(String[] args) {
        // Sub-arrays can be zero-length
        int[][] withEmpty = new int[3][];
        withEmpty[0] = new int[0];  // empty
        withEmpty[1] = new int[5];
        withEmpty[2] = new int[0];  // empty
        
        assert withEmpty[0].length == 0 : "len0";
        assert withEmpty[1].length == 5 : "len1";
        assert withEmpty[2].length == 0 : "len2";
        
        // Can still access the non-empty row
        withEmpty[1][0] = 42;
        assert withEmpty[1][0] == 42 : "access";
        
        System.out.println("Jagged empty test passed.");
    }
}
