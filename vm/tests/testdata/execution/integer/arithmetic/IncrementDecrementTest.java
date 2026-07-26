// @test feature = "execution.integer.arithmetic"
// @test description = "Verifies compound arithmetic and integer increment and decrement expressions."
// @test category = "success"

package execution.integer.arithmetic;

public class IncrementDecrementTest {
    public static void main(String[] args) {
        int compound = 1;
        compound += Integer.MAX_VALUE;
        assert compound == Integer.MIN_VALUE : "compound.wrap";

        int value = 1;
        assert value++ == 1 : "post.inc";

        value = 1;
        assert value-- == 1 : "post.dec";

        value = 1;
        assert ++value == 2 : "pre.inc";

        value = 1;
        assert --value == 0 : "pre.dec";
    }
}
