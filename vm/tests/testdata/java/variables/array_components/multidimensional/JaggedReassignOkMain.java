package variables.array_components.multidimensional.jagged_reassign;

public class JaggedReassignOkMain {
    public static void main(String[] args) {
        int[][] jagged = new int[2][];
        
        // First assignment
        jagged[0] = new int[3];
        jagged[0][0] = 1;
        jagged[0][1] = 2;
        jagged[0][2] = 3;
        
        // Reassign to different size
        jagged[0] = new int[5];
        assert jagged[0].length == 5 : "reassign.len";
        assert jagged[0][0] == 0 : "reassign.default0";
        assert jagged[0][4] == 0 : "reassign.default4";
        
        // Can reassign to smaller
        jagged[0] = new int[1];
        assert jagged[0].length == 1 : "smaller.len";
        
        System.out.println("Jagged reassign test passed.");
    }
}
