// @test feature = "exceptions.throwing"
// @test description = "Verifies explicit throws preserve identity and message, while throwing null produces NullPointerException."
// @test category = "success"

package exceptions.throwing;

public class ThrowingTest {
    public static void main(String[] args) {
        IllegalArgumentException original = new IllegalArgumentException("original");
        try {
            throw original;
        } catch (IllegalArgumentException caught) {
            assert caught == original : "throwing.identity";
            assert "original".equals(caught.getMessage()) : "throwing.message";
        }

        try {
            throw (RuntimeException) runtimeNull();
        } catch (NullPointerException expected) {
            assert expected != null : "throwing.null";
        }
    }

    static Object runtimeNull() {
        return null;
    }
}
