// @test feature = "exceptions.uncaught-reporting"
// @test description = "Verifies an exception escaping main reports its type, message, and caller frames before failure."
// @test category = "error"

package exceptions.reporting;

public class UncaughtExceptionTest {
    public static void main(String[] args) {
        fail();
    }

    static void fail() {
        throw new IllegalStateException("uncaught-test");
    }
}
