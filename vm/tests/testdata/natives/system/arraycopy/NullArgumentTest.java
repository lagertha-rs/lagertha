// @test feature = "natives.system.arraycopy"
// @test description = "Verifies null source and destination arguments throw NullPointerException without copying."
// @test category = "success"

package natives.system.arraycopy;

public class NullArgumentTest {
    public static void main(String[] args) {
        int[] destination = { 7 };
        try {
            System.arraycopy(null, 0, destination, 0, 0);
            assert false : "arraycopy.null.source.missing.exception";
        } catch (NullPointerException expected) {
            assert destination[0] == 7 : "arraycopy.null.source.unchanged";
        }

        try {
            System.arraycopy(new int[] { 1 }, 0, null, 0, 0);
            assert false : "arraycopy.null.destination.missing.exception";
        } catch (NullPointerException expected) {
            assert destination[0] == 7 : "arraycopy.null.destination";
        }

        try {
            System.arraycopy(null, 0, null, 0, 0);
            assert false : "arraycopy.null.both.missing.exception";
        } catch (NullPointerException expected) {
            assert destination[0] == 7 : "arraycopy.null.both";
        }
    }
}
