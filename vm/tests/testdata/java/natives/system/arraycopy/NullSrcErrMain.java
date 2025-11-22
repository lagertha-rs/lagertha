package natives.system.arraycopy.null_src;

public class NullSrcErrMain {
    public static void main(String[] args) {
        Object[] dst = new Object[1];
        // Null src → NPE thrown by the native
        System.arraycopy(null, 0, dst, 0, 1);
    }
}