package natives.java.lang.system_arraycopy_should_throw_with_non_arr_src_err;

public class SystemArrayCopyShouldThrowWithNonArrSrcErrMain {
    public static void main(String[] args) {
        Object g = new Object();
        int[] arr = {1, 2, 3};
        // Non array source
        System.arraycopy(g, 0, arr, 0, 0);
    }
}