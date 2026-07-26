// @test feature = "natives.system.arraycopy"
// @test description = "Verifies non-array source and destination arguments throw ArrayStoreException at zero length."
// @test category = "success"

package natives.system.arraycopy;

public class NonArrayArgumentTest {
    public static void main(String[] args) {
        int[] destination = { 5 };
        try {
            System.arraycopy(new Object(), 0, destination, 0, 0);
            assert false : "arraycopy.nonarray.source.missing.exception";
        } catch (ArrayStoreException expected) {
            assert destination[0] == 5 : "arraycopy.nonarray.source.unchanged";
        }

        try {
            System.arraycopy(new int[] { 1 }, 0, new Object(), 0, 0);
            assert false : "arraycopy.nonarray.destination.missing.exception";
        } catch (ArrayStoreException expected) {
            assert destination[0] == 5 : "arraycopy.nonarray.destination";
        }
    }
}
