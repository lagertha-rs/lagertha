// @test feature = "execution.long.arithmetic"
// @test description = "Verifies long overflow wrapping for binary and unary arithmetic."
// @test category = "success"

package execution.longs.arithmetic;

public class OverflowTest {
    public static void main(String[] args) {
        long maximum = Long.MAX_VALUE;
        long minimum = Long.MIN_VALUE;
        long factor = 0x100000000L;

        assert maximum + 1L == minimum : "add.wrap";
        assert minimum - 1L == maximum : "sub.wrap";
        assert factor * factor == 0L : "mul.wrap";
        assert -minimum == minimum : "neg.edge";
    }
}
