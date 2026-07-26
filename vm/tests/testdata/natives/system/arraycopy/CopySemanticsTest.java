// @test feature = "natives.system.arraycopy"
// @test description = "Verifies full, partial, zero-length, overlapping, primitive, and reference array copies."
// @test category = "success"

package natives.system.arraycopy;

public class CopySemanticsTest {
    public static void main(String[] args) {
        int[] source = { 1, 2, 3, 4, 5 };
        int[] destination = new int[5];
        System.arraycopy(source, 0, destination, 0, source.length);
        assert destination[0] == 1 : "arraycopy.full.first";
        assert destination[4] == 5 : "arraycopy.full.last";

        int[] partial = { 9, 9, 9, 9, 9 };
        System.arraycopy(source, 1, partial, 1, 3);
        assert partial[0] == 9 : "arraycopy.partial.prefix";
        assert partial[1] == 2 : "arraycopy.partial.first";
        assert partial[3] == 4 : "arraycopy.partial.last";
        assert partial[4] == 9 : "arraycopy.partial.suffix";

        String[] references = { "a", "b", null };
        String[] referenceDestination = new String[3];
        System.arraycopy(references, 0, referenceDestination, 0, references.length);
        assert referenceDestination[0] == references[0] : "arraycopy.reference.identity";
        assert referenceDestination[2] == null : "arraycopy.reference.null";

        int[] unchanged = { 6, 7 };
        System.arraycopy(source, 0, unchanged, 0, 0);
        assert unchanged[0] == 6 && unchanged[1] == 7 : "arraycopy.zero.length";

        int[] overlapRight = { 0, 1, 2, 3, 4, 5 };
        System.arraycopy(overlapRight, 0, overlapRight, 2, 4);
        assert overlapRight[2] == 0 : "arraycopy.overlap.right.first";
        assert overlapRight[5] == 3 : "arraycopy.overlap.right.last";

        int[] overlapLeft = { 0, 1, 2, 3, 4, 5 };
        System.arraycopy(overlapLeft, 2, overlapLeft, 0, 4);
        assert overlapLeft[0] == 2 : "arraycopy.overlap.left.first";
        assert overlapLeft[3] == 5 : "arraycopy.overlap.left.last";
    }
}
