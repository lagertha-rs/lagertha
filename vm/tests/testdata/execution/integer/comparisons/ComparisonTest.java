// @test feature = "execution.integer.comparisons"
// @test description = "Verifies signed integer ordering, equality, and inequality."
// @test category = "success"

package execution.integer.comparisons;

public class ComparisonTest {
    public static void main(String[] args) {
        int negative = -1;
        int positive = 1;
        int minimum = Integer.MIN_VALUE;
        int maximum = Integer.MAX_VALUE;
        int value = 123456789;
        int sameValue = 123456789;
        int differentValue = -123456789;
        assert negative < positive : "cmp.signed.lt";
        assert minimum <= maximum : "cmp.signed.ge";
        assert value == sameValue : "cmp.eq";
        assert value != differentValue : "cmp.ne";
    }
}
