package variables.array_components.regular.int_array;

public class IntArrayOkMain {
    public static void main(String[] args) {
        int[] arr = new int[3];
        arr[0] = 0;
        arr[1] = Integer.MAX_VALUE;
        arr[2] = Integer.MIN_VALUE;
        
        assert arr[0] == 0 : "int.0";
        assert arr[1] == 2147483647 : "int.max";
        assert arr[2] == -2147483648 : "int.min";
        assert arr.length == 3 : "int.length";
        
        System.out.println("Int array test passed.");
    }
}
