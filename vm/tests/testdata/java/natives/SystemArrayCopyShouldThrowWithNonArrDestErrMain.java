package natives.java.lang.system_arraycopy_should_throw_with_non_dest_src_err;

public class SystemArrayCopyShouldThrowWithNonArrDestErrMain {
    public static void main(String[] args) {
        Object g = new Object();
        int[] arr = {1, 2, 3};
        // Non array dest
        System.arraycopy(arr, 0, g, 0, 0);
    }
}