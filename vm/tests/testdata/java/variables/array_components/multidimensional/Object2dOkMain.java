package variables.array_components.multidimensional.object_2d;

public class Object2dOkMain {
    public static void main(String[] args) {
        Object[][] matrix = new Object[2][3];
        
        assert matrix.length == 2 : "rows";
        assert matrix[0].length == 3 : "cols";
        
        // All default to null
        assert matrix[0][0] == null : "default.00";
        assert matrix[1][2] == null : "default.12";
        
        Object obj1 = new Object();
        Object obj2 = new Object();
        matrix[0][0] = obj1;
        matrix[1][2] = obj2;
        
        assert matrix[0][0] == obj1 : "assigned.00";
        assert matrix[1][2] == obj2 : "assigned.12";
        assert matrix[0][0] != matrix[1][2] : "different";
        
        // Can set to null
        matrix[0][0] = null;
        assert matrix[0][0] == null : "nulled";
        
        System.out.println("2D object array test passed.");
    }
}
