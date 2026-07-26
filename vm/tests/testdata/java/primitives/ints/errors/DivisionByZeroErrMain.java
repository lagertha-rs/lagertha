// @test feature = "execution.integer.arithmetic"
// @test description = "Verifies that integer division by zero throws ArithmeticException."
// @test category = "error"

package primitives.ints.errors.division_by_zero;

public class DivisionByZeroErrMain {
    public static void main(String[] args) {
        var a = 1 / 0;
    }
}
