package natives.system.arraycopy.null_dest;

public class NullDestErrMain {
    public static void main(String[] args) {
        Object[] src = new Object[1];
        // Null dest → NPE thrown by the native
        System.arraycopy(src, 0, null, 0, 1);
    }
}