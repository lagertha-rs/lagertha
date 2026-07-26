// @test feature = "execution.long.conversions"
// @test description = "Verifies widening integers to long and narrowing long values to integers."
// @test category = "success"

package execution.longs.conversions;

public class LongConversionTest {
    public static void main(String[] args) {
        int positiveInteger = 100;
        int negativeInteger = -100;
        assert (long) positiveInteger == 100L : "widen.i2l.positive";
        assert (long) negativeInteger == -100L : "widen.i2l.negative";

        long integerMaximum = 2147483647L;
        long signBoundary = 2147483648L;
        long lowBitsSet = 4294967295L;
        long fullCycle = 4294967296L;
        long longMaximum = Long.MAX_VALUE;
        assert (int) integerMaximum == Integer.MAX_VALUE : "narrow.l2i.max";
        assert (int) signBoundary == Integer.MIN_VALUE : "narrow.l2i.sign";
        assert (int) lowBitsSet == -1 : "narrow.l2i.low.bits";
        assert (int) fullCycle == 0 : "narrow.l2i.cycle";
        assert (int) longMaximum == -1 : "narrow.l2i.long.max";
    }
}
