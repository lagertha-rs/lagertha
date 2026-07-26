// @test feature = "execution.arrays.allocation-exceptions"
// @test description = "Verifies negative primitive and reference array lengths throw NegativeArraySizeException."
// @test category = "success"

package execution.arrays.errors;

public class NegativeArraySizeTest {
    public static void main(String[] args) {
        int negative = runtime(-1);

        try {
            int[] ignored = new int[negative];
            assert ignored.length < 0 : "allocation.primitive.missing.exception";
        } catch (NegativeArraySizeException expected) {
            assert negative == -1 : "allocation.primitive.negative";
        }

        try {
            Object[] ignored = new Object[negative];
            assert ignored.length < 0 : "allocation.reference.missing.exception";
        } catch (NegativeArraySizeException expected) {
            assert negative == -1 : "allocation.reference.negative";
        }
    }

    static int runtime(int value) {
        return value;
    }
}
