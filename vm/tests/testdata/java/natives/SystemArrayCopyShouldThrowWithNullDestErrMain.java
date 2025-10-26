package natives.java.lang.system_arraycopy_should_throw_with_null_dest_err;

public class SystemArrayCopyShouldThrowWithNullDestErrMain {
    public static void main(String[] args) {
        Object[] src = new Object[1];
        // Null dest → NPE thrown by the native
        System.arraycopy(src, 0, null, 0, 1);
    }
}