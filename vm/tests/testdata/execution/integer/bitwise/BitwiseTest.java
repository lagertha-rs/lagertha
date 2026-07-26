// @test feature = "execution.integer.bitwise"
// @test description = "Verifies integer complement, conjunction, disjunction, and exclusive-or."
// @test category = "success"

package execution.integer.bitwise;

public class BitwiseTest {
    public static void main(String[] args) {
        int zero = 0;
        int left = 0xAA55AA55;
        int right = 0x0F0F0F0F;
        assert ~zero == -1 : "bit.not";
        assert (left & right) == 0x0A050A05 : "bit.and";
        assert (left | right) == 0xAF5FAF5F : "bit.or";
        assert (left ^ left) == 0 : "bit.xor.self";
    }
}
