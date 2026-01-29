package variables.array_components.regular.float_array;

public class FloatArrayOkMain {
    public static void main(String[] args) {
        float[] arr = new float[3];
        arr[0] = 0.0f;
        arr[1] = 3.14f;
        arr[2] = -1.5f;
        
        assert arr[0] == 0.0f : "float.0";
        assert arr[1] == 3.14f : "float.pi";
        assert arr[2] == -1.5f : "float.neg";
        assert arr.length == 3 : "float.length";
        
        System.out.println("Float array test passed.");
    }
}
