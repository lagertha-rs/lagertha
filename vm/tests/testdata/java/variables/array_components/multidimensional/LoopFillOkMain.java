package variables.array_components.multidimensional.loop_fill;

public class LoopFillOkMain {
    public static void main(String[] args) {
        int[][] matrix = new int[3][4];
        
        // Fill with row*10 + col
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 4; j++) {
                matrix[i][j] = i * 10 + j;
            }
        }
        
        assert matrix[0][0] == 0 : "00";
        assert matrix[0][3] == 3 : "03";
        assert matrix[1][0] == 10 : "10";
        assert matrix[1][3] == 13 : "13";
        assert matrix[2][0] == 20 : "20";
        assert matrix[2][3] == 23 : "23";
        
        // Sum all
        int sum = 0;
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 4; j++) {
                sum += matrix[i][j];
            }
        }
        // 0+1+2+3 + 10+11+12+13 + 20+21+22+23 = 6 + 46 + 86 = 138
        assert sum == 138 : "sum";
        
        System.out.println("Loop fill test passed.");
    }
}
