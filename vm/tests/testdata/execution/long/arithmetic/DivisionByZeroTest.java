// @test feature = "execution.long.arithmetic"
// @test description = "Verifies that long division by zero throws ArithmeticException."
// @test category = "error"

package execution.longs.arithmetic;

public class DivisionByZeroTest {
    public static void main(String[] args) {
        long dividend = 1L;
        long divisor = 0L;
        var result = dividend / divisor;
    }
}
