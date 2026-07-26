// @test feature = "execution.integer.arithmetic"
// @test description = "Verifies that integer division by zero throws ArithmeticException."
// @test category = "error"

package execution.integer.arithmetic;

public class DivisionByZeroTest {
    public static void main(String[] args) {
        var result = 1 / 0;
    }
}
