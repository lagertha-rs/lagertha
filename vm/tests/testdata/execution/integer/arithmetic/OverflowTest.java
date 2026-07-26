// @test feature = "execution.integer.arithmetic"
// @test description = "Verifies integer overflow wrapping for binary and unary arithmetic."
// @test category = "success"

package execution.integer.arithmetic;

public class OverflowTest {
    public static void main(String[] args) {
        int maximum = Integer.MAX_VALUE;
        int minimum = Integer.MIN_VALUE;

        assert maximum + 1 == minimum : "add.wrap";
        assert minimum - 1 == maximum : "sub.wrap";
        assert 65536 * 65536 == 0 : "mul.wrap";
        assert -minimum == minimum : "neg.edge";
    }
}
