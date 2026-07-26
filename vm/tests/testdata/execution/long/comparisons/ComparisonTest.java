// @test feature = "execution.long.comparisons"
// @test description = "Verifies signed long ordering, equality, and inequality."
// @test category = "success"

package execution.longs.comparisons;

public class ComparisonTest {
    public static void main(String[] args) {
        long negative = -1L;
        long positive = 1L;
        long minimum = Long.MIN_VALUE;
        long maximum = Long.MAX_VALUE;
        long value = 123456789L;
        long sameValue = 123456789L;
        long differentValue = -123456789L;
        assert negative < positive : "cmp.signed.lt";
        assert minimum <= maximum : "cmp.signed.ge";
        assert value == sameValue : "cmp.eq";
        assert value != differentValue : "cmp.ne";
    }
}
