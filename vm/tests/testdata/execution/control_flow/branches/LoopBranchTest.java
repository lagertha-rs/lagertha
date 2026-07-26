// @test feature = "execution.control-flow.conditional-branches"
// @test description = "Verifies conditional branches across while, do-while, for, and nested loops."
// @test category = "success"

package execution.controlflow.branches;

public class LoopBranchTest {
    public static void main(String[] args) {
        int count = 0;
        int sum = 0;
        while (count < 5) {
            sum += count;
            count++;
        }
        assert sum == 10 : "while.sum";
        assert count == 5 : "while.count";

        int zeroIterations = 0;
        while (zeroIterations > 0) {
            zeroIterations = -1;
        }
        assert zeroIterations == 0 : "while.zero.iterations";

        int once = 0;
        do {
            once++;
        } while (false);
        assert once == 1 : "do.while.once";

        int nested = 0;
        for (int outer = 0; outer < 3; outer++) {
            for (int inner = 0; inner < 2; inner++) {
                nested++;
            }
        }
        assert nested == 6 : "for.nested";

        int left;
        int right;
        for (left = 0, right = 10; left < right; left++, right--) {
        }
        assert left == 5 && right == 5 : "for.multiple.update";
    }
}
