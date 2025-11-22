package natives.system.arraycopy.non_array_src;

public class NonArraySrcErrMain {
    public static void main(String[] args) {
        Object g = new Object();
        int[] arr = {1, 2, 3};
        // Non array source
        System.arraycopy(g, 0, arr, 0, 0);
    }
}