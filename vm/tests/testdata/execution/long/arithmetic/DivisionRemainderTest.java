// @test feature = "execution.long.arithmetic"
// @test description = "Verifies signed long division and remainder semantics and edge cases."
// @test category = "success"

package execution.longs.arithmetic;

public class DivisionRemainderTest {
    public static void main(String[] args) {
        long positive = 7L;
        long negative = -7L;
        long divisor = 3L;
        long negativeDivisor = -3L;
        assert positive / divisor == 2L : "div.trunc.pos";
        assert negative / divisor == -2L : "div.trunc.negA";
        assert positive / negativeDivisor == -2L : "div.trunc.negB";
        assert positive % divisor == 1L : "rem.sign.pos";
        assert negative % divisor == -1L : "rem.sign.negA";
        assert positive % negativeDivisor == 1L : "rem.sign.negB";
        assert negative % negativeDivisor == -1L : "rem.sign.negBoth";

        long dividend = -123456789L;
        long identityDivisor = 67L;
        assert (dividend / identityDivisor) * identityDivisor + dividend % identityDivisor == dividend
                : "divrem.identity";
        long minimum = Long.MIN_VALUE;
        long minusOne = -1L;
        assert minimum / minusOne == minimum : "min.div.minus1";
        assert minimum % minusOne == 0L : "min.rem.minus1";
    }
}
