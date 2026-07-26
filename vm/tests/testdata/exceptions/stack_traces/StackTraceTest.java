// @test feature = "exceptions.stack-traces"
// @test description = "Verifies explicit stack-trace output contains the throw site and ordered Java callers."
// @test category = "success"

package exceptions.stack_traces;

public class StackTraceTest {
    public static void main(String[] args) {
        try {
            first();
        } catch (IllegalStateException caught) {
            caught.printStackTrace();
        }
    }

    static void first() {
        second();
    }

    static void second() {
        throw new IllegalStateException("stack-trace-test");
    }
}
