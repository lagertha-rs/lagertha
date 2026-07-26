// @test feature = "execution.frames.recursion"
// @test description = "Verifies isolated parameters and pending results across branching recursive frames."
// @test category = "success"

package execution.frames;

public class RecursiveFrameTest {
    public static void main(String[] args) {
        assert fibonacci(0) == 0 : "recursion.base.zero";
        assert fibonacci(1) == 1 : "recursion.base.one";
        assert fibonacci(8) == 21 : "recursion.branching";
    }

    static int fibonacci(int value) {
        if (value < 2) {
            return value;
        }
        return fibonacci(value - 1) + fibonacci(value - 2);
    }
}
