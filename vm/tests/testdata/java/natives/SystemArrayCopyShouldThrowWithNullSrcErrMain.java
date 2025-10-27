package natives.java.lang.system_arraycopy_should_throw_with_null_src_err;

public class SystemArrayCopyShouldThrowWithNullSrcErrMain {
    public static void main(String[] args) {
        Object[] dst = new Object[1];
        // Null src → NPE thrown by the native
        System.arraycopy(null, 0, dst, 0, 1);
    }
}