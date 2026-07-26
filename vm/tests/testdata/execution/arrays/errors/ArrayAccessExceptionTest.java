// @test feature = "execution.arrays.access-exceptions"
// @test description = "Verifies null and out-of-bounds array loads and stores leave valid components unchanged."
// @test category = "success"

package execution.arrays.errors;

public class ArrayAccessExceptionTest {
    public static void main(String[] args) {
        int[] integers = new int[] { 7, 11 };

        try {
            int ignored = integers[runtime(-1)];
            assert ignored == 0 : "access.negative.load.missing.exception";
        } catch (ArrayIndexOutOfBoundsException expected) {
            assert integers[0] == 7 : "access.negative.load";
        }

        try {
            int ignored = integers[integers.length];
            assert ignored == 0 : "access.high.load.missing.exception";
        } catch (ArrayIndexOutOfBoundsException expected) {
            assert integers[1] == 11 : "access.high.load";
        }

        try {
            integers[runtime(-1)] = 13;
            assert false : "access.negative.store.missing.exception";
        } catch (ArrayIndexOutOfBoundsException expected) {
            assert integers[0] == 7 : "access.negative.store.atomic";
        }

        Object marker = new Object();
        Object[] references = new Object[] { marker };
        try {
            references[references.length] = new Object();
            assert false : "access.high.store.missing.exception";
        } catch (ArrayIndexOutOfBoundsException expected) {
            assert references[0] == marker : "access.high.store.atomic";
        }

        int[] nullIntegers = null;
        try {
            int ignored = nullIntegers[0];
            assert ignored == 0 : "access.null.load.missing.exception";
        } catch (NullPointerException expected) {
            assert nullIntegers == null : "access.null.load";
        }

        Object[] nullReferences = null;
        try {
            nullReferences[0] = marker;
            assert false : "access.null.store.missing.exception";
        } catch (NullPointerException expected) {
            assert nullReferences == null : "access.null.store";
        }

        int[][] rows = new int[1][];
        try {
            int ignored = rows[0][0];
            assert ignored == 0 : "access.null.row.missing.exception";
        } catch (NullPointerException expected) {
            assert rows[0] == null : "access.null.row";
        }
    }

    static int runtime(int value) {
        return value;
    }
}
