// @test feature = "execution.integer.bitwise"
// @test description = "Verifies integer shifts, signedness, and shift-distance masking."
// @test category = "success"

package execution.integer.bitwise;

public class ShiftTest {
    public static void main(String[] args) {
        int one = 1;
        int negativeTwo = -2;
        int count32 = 32;
        int count33 = 33;
        int negativeCount = -1;
        assert one << count32 == 1 : "shl.mask.32";
        assert one << count33 == 2 : "shl.mask.33";
        assert negativeTwo >> one == -1 : "shr.arith.neg";
        assert negativeTwo >>> one == 0x7FFFFFFF : "shr.logic.neg";
        assert one << negativeCount == one << 31 : "shl.negative.count";
        assert one >> count32 == 1 : "shr.mask.same";
    }
}
