package variables.array_components.regular.boolean_array;

public class BooleanArrayOkMain {
    public static void main(String[] args) {
        boolean[] arr = new boolean[3];
        arr[0] = true;
        arr[1] = false;
        arr[2] = true;
        
        assert arr[0] == true : "bool.0";
        assert arr[1] == false : "bool.1";
        assert arr[2] == true : "bool.2";
        assert arr.length == 3 : "bool.length";
        
        System.out.println("Boolean array test passed.");
    }
}
