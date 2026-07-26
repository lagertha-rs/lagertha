// @test feature = "natives.binding"
// @test description = "Verifies an unbound native method throws UnsatisfiedLinkError with a native stack frame."
// @test category = "error"

package natives.binding;

public class MissingNativeTest {
    static native void missing();

    public static void main(String[] args) {
        missing();
    }
}
