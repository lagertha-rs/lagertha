package control_flow.branching.labels;

public class BreakContinueOkMain {
    public static void main(String[] args) {
        // Labeled break from nested loop
        outer: for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                if (i == 1 && j == 1) {
                    break outer;
                }
            }
        }
        // If break outer worked, we exit both loops when i=1, j=1
        
        // Labeled continue
        int sum = 0;
        outerLoop: for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                if (j == 1) {
                    continue outerLoop; // skip remaining j iterations, go to next i
                }
                sum++;
            }
        }
        // For i=0, j=0 sum++, j=1 continue, j=2 skipped
        // For i=1, same pattern
        // For i=2, same pattern
        // Total: 3 iterations of i * 1 increment each = 3
        assert sum == 3 : "labeled continue";
        
        // Simple break in switch (switch not implemented yet, skip)
        
        // Break in while loop with label
        int count = 0;
        labeledWhile: while (true) {
            count++;
            if (count >= 5) {
                break labeledWhile;
            }
        }
        assert count == 5 : "labeled break in while";
        
        // Continue in while with label
        int val = 0;
        int iterations = 0;
        labeledWhile2: while (val < 10) {
            val++;
            if (val % 2 == 0) {
                continue labeledWhile2;
            }
            iterations++;
        }
        assert iterations == 5 : "labeled continue in while (odd numbers)";
        
        // Break from nested blocks
        block: {
            int x = 42;
            if (x > 0) {
                break block;
            }
            // This should not be executed
            assert false : "should not reach here";
        }
        
        System.out.println("Break/continue tests passed.");
    }
}