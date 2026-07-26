// @test feature = "execution.integer.arithmetic"
// @test description = "Verifies that integer remainder by zero throws ArithmeticException."
// @test category = "error"

package execution.integer.arithmetic;

public class RemainderByZeroTest {
    public static void main(String[] args) {
        int dividend = 1;
        int divisor = 0;
        var result = dividend % divisor;
    }
}
