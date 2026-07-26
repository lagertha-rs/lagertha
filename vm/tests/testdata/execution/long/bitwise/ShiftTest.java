// @test feature = "execution.long.bitwise"
// @test description = "Verifies long shifts, signedness, and shift-distance masking."
// @test category = "success"

package execution.longs.bitwise;

public class ShiftTest {
    public static void main(String[] args) {
        long one = 1L;
        long negativeTwo = -2L;
        int count64 = 64;
        int count65 = 65;
        int negativeCount = -1;
        assert one << count64 == 1L : "shl.mask.64";
        assert one << count65 == 2L : "shl.mask.65";
        assert negativeTwo >> 1 == -1L : "shr.arith.neg";
        assert negativeTwo >>> 1 == 0x7FFFFFFFFFFFFFFFL : "shr.logic.neg";
        assert one << negativeCount == one << 63 : "shl.negative.count";
        assert one >> count64 == 1L : "shr.mask.same";
    }
}
