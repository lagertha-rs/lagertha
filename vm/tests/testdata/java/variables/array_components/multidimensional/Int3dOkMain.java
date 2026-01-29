package variables.array_components.multidimensional.int_3d;

public class Int3dOkMain {
    public static void main(String[] args) {
        int[][][] cube = new int[2][3][4];
        
        assert cube.length == 2 : "dim1";
        assert cube[0].length == 3 : "dim2";
        assert cube[0][0].length == 4 : "dim3";
        
        // Write corners
        cube[0][0][0] = 1;
        cube[1][2][3] = 999;
        
        assert cube[0][0][0] == 1 : "origin";
        assert cube[1][2][3] == 999 : "far";
        assert cube[0][1][1] == 0 : "default";
        
        System.out.println("3D int array test passed.");
    }
}
