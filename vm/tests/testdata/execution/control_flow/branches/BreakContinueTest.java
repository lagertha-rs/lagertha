// @test feature = "execution.control-flow.unconditional-branches"
// @test description = "Verifies break and continue transfers across simple, nested, and labeled regions."
// @test category = "success"

package execution.controlflow.branches;

public class BreakContinueTest {
    public static void main(String[] args) {
        int breakCount = 0;
        while (true) {
            breakCount++;
            if (breakCount == 10) {
                break;
            }
        }
        assert breakCount == 10 : "break.simple";

        int oddCount = 0;
        for (int value = 1; value <= 10; value++) {
            if (value % 2 == 0) {
                continue;
            }
            oddCount++;
        }
        assert oddCount == 5 : "continue.simple";

        int stopOuter = -1;
        int stopInner = -1;
        outer: for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                if (i == 1 && j == 1) {
                    stopOuter = i;
                    stopInner = j;
                    break outer;
                }
            }
        }
        assert stopOuter == 1 && stopInner == 1 : "break.labeled.nested";

        int labeledCount = 0;
        outerLoop: for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                if (j == 1) {
                    continue outerLoop;
                }
                labeledCount++;
            }
        }
        assert labeledCount == 3 : "continue.labeled";

        block: {
            int positive = 42;
            if (positive > 0) {
                break block;
            }
            assert false : "break.block.unreachable";
        }
    }
}
