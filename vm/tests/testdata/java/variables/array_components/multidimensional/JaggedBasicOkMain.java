package variables.array_components.multidimensional.jagged_basic;

public class JaggedBasicOkMain {
    public static void main(String[] args) {
        // Rows with different lengths
        int[][] jagged = new int[3][];
        jagged[0] = new int[2];
        jagged[1] = new int[4];
        jagged[2] = new int[1];

        assert jagged[0].length == 2 : "row0.len";
        assert jagged[1].length == 4 : "row1.len";
        assert jagged[2].length == 1 : "row2.len";

        jagged[0][0] = 1;
        jagged[0][1] = 2;
        jagged[1][3] = 40;
        jagged[2][0] = 100;

        assert jagged[0][0] == 1 : "00";
        assert jagged[0][1] == 2 : "01";
        assert jagged[1][3] == 40 : "13";
        assert jagged[2][0] == 100 : "20";

        System.out.println("Jagged basic test passed.");
    }
}
