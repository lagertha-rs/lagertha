package variables.local.loop_scoping;

public class LoopScopingOkMain {
    public static void main(String[] args) {
        // For loop variable scope
        int sum = 0;
        for (int i = 0; i < 5; i++) {
            sum += i;
            assert i >= 0 && i < 5 : "for.i.range";
        }
        assert sum == 10 : "for.sum"; // 0+1+2+3+4 = 10

        // Nested for loops with same variable name
        int product = 1;
        for (int i = 1; i <= 3; i++) {
            for (int j = 1; j <= 2; j++) {
                product *= i;
            }
        }
        assert product == 36 : "nested.for.product"; // 1*1*2*2*3*3 = 36

        // While loop with local
        int whileCounter = 0;
        int whileLimit = 3;
        while (whileCounter < whileLimit) {
            whileCounter++;
        }
        assert whileCounter == 3 : "while.counter";

        // Do-while with local
        int doCounter = 0;
        do {
            doCounter++;
        } while (doCounter < 3);
        assert doCounter == 3 : "dowhile.counter";

        // Variable declared in loop body
        for (int i = 0; i < 3; i++) {
            int loopLocal = i * 10;
            assert loopLocal == i * 10 : "loopbody.local";
        }

        System.out.println("All loop scoping tests passed.");
    }
}
