// @test feature = "execution.frames.local-variables"
// @test description = "Verifies local slots for each JVM computational value kind and references."
// @test category = "success"

package execution.frames;

public class LocalVariableTest {
    public static void main(String[] args) {
        int integer = -123456789;
        float floating = 1.25f;
        long longValue = Long.MIN_VALUE + 42L;
        double doubleValue = -2.5d;
        Object reference = new Object();
        int[] array = new int[] { 10, 20 };

        assert integer == -123456789 : "local.int";
        assert floating == 1.25f : "local.float";
        assert longValue == Long.MIN_VALUE + 42L : "local.long";
        assert doubleValue == -2.5d : "local.double";
        assert reference != null : "local.reference";
        assert array[1] == 20 : "local.array.reference";

        reference = null;
        assert reference == null : "local.null.reference";
    }
}
