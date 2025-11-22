package natives.system.arraycopy.non_array_dest;

public class NonArrayDestErrMain {
    public static void main(String[] args) {
        Object g = new Object();
        int[] arr = {1, 2, 3};
        // Non array dest
        System.arraycopy(arr, 0, g, 0, 0);
    }
}