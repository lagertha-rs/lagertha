package variables.array_components.multidimensional.jagged_3d;

public class Jagged3dOkMain {
    public static void main(String[] args) {
        // 3D jagged array
        int[][][] jagged3d = new int[2][][];
        assert jagged3d[0] == null : "null.0";
        assert jagged3d[1] == null : "null.1";
        
        // First plane: 2x3
        jagged3d[0] = new int[2][];
        jagged3d[0][0] = new int[3];
        jagged3d[0][1] = new int[3];
        
        // Second plane: 1x5
        jagged3d[1] = new int[1][];
        jagged3d[1][0] = new int[5];
        
        assert jagged3d[0].length == 2 : "plane0.rows";
        assert jagged3d[0][0].length == 3 : "plane0.row0.cols";
        assert jagged3d[1].length == 1 : "plane1.rows";
        assert jagged3d[1][0].length == 5 : "plane1.row0.cols";
        
        jagged3d[0][0][0] = 1;
        jagged3d[0][1][2] = 99;
        jagged3d[1][0][4] = 500;
        
        assert jagged3d[0][0][0] == 1 : "000";
        assert jagged3d[0][1][2] == 99 : "012";
        assert jagged3d[1][0][4] == 500 : "104";
        
        System.out.println("Jagged 3D test passed.");
    }
}
