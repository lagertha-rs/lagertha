// @test feature = "execution.integer.arithmetic"
// @test description = "Verifies signed integer division and remainder semantics and edge cases."
// @test category = "success"

package execution.integer.arithmetic;

public class DivisionRemainderTest {
    public static void main(String[] args) {
        int positive = 7;
        int negative = -7;
        int divisor = 3;
        int negativeDivisor = -3;
        assert positive / divisor == 2 : "div.trunc.pos";
        assert negative / divisor == -2 : "div.trunc.negA";
        assert positive / negativeDivisor == -2 : "div.trunc.negB";
        assert positive % divisor == 1 : "rem.sign.pos";
        assert negative % divisor == -1 : "rem.sign.negA";
        assert positive % negativeDivisor == 1 : "rem.sign.negB";
        assert negative % negativeDivisor == -1 : "rem.sign.negBoth";

        int dividend = -12345;
        int identityDivisor = 67;
        assert (dividend / identityDivisor) * identityDivisor + dividend % identityDivisor == dividend
                : "divrem.identity";
        int minimum = Integer.MIN_VALUE;
        int minusOne = -1;
        assert minimum / minusOne == minimum : "min.div.minus1";
        assert minimum % minusOne == 0 : "min.rem.minus1";
    }
}
