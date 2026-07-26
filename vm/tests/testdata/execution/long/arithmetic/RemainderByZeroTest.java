// @test feature = "execution.long.arithmetic"
// @test description = "Verifies that long remainder by zero throws ArithmeticException."
// @test category = "error"

package execution.longs.arithmetic;

public class RemainderByZeroTest {
    public static void main(String[] args) {
        long dividend = 1L;
        long divisor = 0L;
        var result = dividend % divisor;
    }
}
