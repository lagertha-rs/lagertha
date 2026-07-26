// @test feature = "execution.long.arithmetic"
// @test description = "Verifies compound arithmetic and long increment and decrement expressions."
// @test category = "success"

package execution.longs.arithmetic;

public class IncrementDecrementTest {
    public static void main(String[] args) {
        long compound = 1L;
        compound += Long.MAX_VALUE;
        assert compound == Long.MIN_VALUE : "compound.wrap";

        long value = 1L;
        assert value++ == 1L : "post.inc";

        value = 1L;
        assert value-- == 1L : "post.dec";

        value = 1L;
        assert ++value == 2L : "pre.inc";

        value = 1L;
        assert --value == 0L : "pre.dec";
    }
}
