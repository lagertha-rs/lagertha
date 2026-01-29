package variables.array_components.regular.double_array;

public class DoubleArrayOkMain {
    public static void main(String[] args) {
        double[] arr = new double[3];
        arr[0] = 0.0;
        arr[1] = 3.141592653589793;
        arr[2] = -2.718281828;
        
        assert arr[0] == 0.0 : "double.0";
        assert arr[1] == 3.141592653589793 : "double.pi";
        assert arr[2] == -2.718281828 : "double.e";
        assert arr.length == 3 : "double.length";
        
        System.out.println("Double array test passed.");
    }
}
