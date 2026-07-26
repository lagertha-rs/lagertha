// @test feature = "execution.long.bitwise"
// @test description = "Verifies long complement, conjunction, disjunction, and exclusive-or."
// @test category = "success"

package execution.longs.bitwise;

public class BitwiseTest {
    public static void main(String[] args) {
        long zero = 0L;
        long left = 0xAA55AA55AA55AA55L;
        long right = 0x0F0F0F0F0F0F0F0FL;
        assert ~zero == -1L : "bit.not";
        assert (left & right) == 0x0A050A050A050A05L : "bit.and";
        assert (left | right) == 0xAF5FAF5FAF5FAF5FL : "bit.or";
        assert (left ^ left) == 0L : "bit.xor.self";
    }
}
