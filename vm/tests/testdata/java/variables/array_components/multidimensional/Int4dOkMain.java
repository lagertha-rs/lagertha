package variables.array_components.multidimensional.int_4d;

public class Int4dOkMain {
    public static void main(String[] args) {
        int[][][][] hyper = new int[2][2][2][2];
        
        assert hyper.length == 2 : "dim1";
        assert hyper[0].length == 2 : "dim2";
        assert hyper[0][0].length == 2 : "dim3";
        assert hyper[0][0][0].length == 2 : "dim4";
        
        hyper[0][0][0][0] = 1;
        hyper[1][1][1][1] = 16;
        hyper[0][1][0][1] = 42;
        
        assert hyper[0][0][0][0] == 1 : "0000";
        assert hyper[1][1][1][1] == 16 : "1111";
        assert hyper[0][1][0][1] == 42 : "0101";
        assert hyper[1][0][1][0] == 0 : "default";
        
        System.out.println("4D int array test passed.");
    }
}
