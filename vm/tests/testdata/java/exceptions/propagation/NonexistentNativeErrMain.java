package exceptions.propagation.nonexistent_native;

public class NonexistentNativeErrMain {
    static native void fakeNative();

    public static void main(String[] args) {
        fakeNative();
    }
}