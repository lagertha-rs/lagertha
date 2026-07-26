// @test feature = "execution.integer.conversions"
// @test description = "Verifies narrowing after compound assignment and increment or decrement."
// @test category = "success"

package execution.integer.conversions;

public class NarrowCompoundTest {
    public static void main(String[] args) {
        byte byteValue = Byte.MAX_VALUE;
        assert byteValue++ == Byte.MAX_VALUE : "byte.post.inc.result";
        assert byteValue == Byte.MIN_VALUE : "byte.post.inc.wrap";
        byteValue = Byte.MIN_VALUE;
        assert --byteValue == Byte.MAX_VALUE : "byte.pre.dec.wrap";

        short shortValue = Short.MAX_VALUE;
        assert shortValue++ == Short.MAX_VALUE : "short.post.inc.result";
        assert shortValue == Short.MIN_VALUE : "short.post.inc.wrap";
        shortValue = Short.MIN_VALUE;
        assert --shortValue == Short.MAX_VALUE : "short.pre.dec.wrap";

        char charValue = Character.MAX_VALUE;
        charValue += 1;
        assert charValue == Character.MIN_VALUE : "char.compound.wrap";
    }
}
