package variables.array_components.multidimensional.rectangular;

public class RectangularOkMain {
    public static void main(String[] args) {
        // Wide array
        int[][] wide = new int[2][10];
        assert wide.length == 2 : "wide.rows";
        assert wide[0].length == 10 : "wide.cols";
        wide[0][9] = 99;
        wide[1][0] = 11;
        assert wide[0][9] == 99 : "wide.09";
        assert wide[1][0] == 11 : "wide.10";

        // Tall array
        int[][] tall = new int[10][2];
        assert tall.length == 10 : "tall.rows";
        assert tall[0].length == 2 : "tall.cols";
        tall[9][1] = 88;
        tall[0][0] = 22;
        assert tall[9][1] == 88 : "tall.91";
        assert tall[0][0] == 22 : "tall.00";

        // Single row
        int[][] singleRow = new int[1][5];
        assert singleRow.length == 1 : "singleRow.len";
        singleRow[0][4] = 5;
        assert singleRow[0][4] == 5 : "singleRow.04";

        // Single column
        int[][] singleCol = new int[5][1];
        assert singleCol.length == 5 : "singleCol.rows";
        singleCol[4][0] = 50;
        assert singleCol[4][0] == 50 : "singleCol.40";

        // 1x1
        int[][] oneByOne = new int[1][1];
        oneByOne[0][0] = 42;
        assert oneByOne[0][0] == 42 : "1x1";

        System.out.println("Rectangular arrays test passed.");
    }
}
