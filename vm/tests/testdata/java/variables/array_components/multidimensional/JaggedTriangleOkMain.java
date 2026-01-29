package variables.array_components.multidimensional.jagged_triangle;

public class JaggedTriangleOkMain {
    public static void main(String[] args) {
        // Triangular pattern
        int size = 5;
        int[][] triangle = new int[size][];
        for (int i = 0; i < size; i++) {
            triangle[i] = new int[i + 1];
        }
        
        // Verify lengths
        assert triangle[0].length == 1 : "len0";
        assert triangle[1].length == 2 : "len1";
        assert triangle[2].length == 3 : "len2";
        assert triangle[3].length == 4 : "len3";
        assert triangle[4].length == 5 : "len4";
        
        // Fill with row*10 + col
        for (int i = 0; i < size; i++) {
            for (int j = 0; j <= i; j++) {
                triangle[i][j] = i * 10 + j;
            }
        }
        
        assert triangle[0][0] == 0 : "00";
        assert triangle[1][0] == 10 : "10";
        assert triangle[1][1] == 11 : "11";
        assert triangle[4][0] == 40 : "40";
        assert triangle[4][4] == 44 : "44";
        
        System.out.println("Jagged triangle test passed.");
    }
}
