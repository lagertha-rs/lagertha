package control_flow.loops.do_while;

public class DoWhileLoopOkMain {
    public static void main(String[] args) {
        // Basic do-while (executes at least once)
        int count = 0;
        int sum = 0;
        do {
            sum += count;
            count++;
        } while (count < 5);
        assert sum == 10 : "do-while sum 0..4 = 10";
        assert count == 5 : "do-while executes 5 times";
        
        // Do-while with false condition (executes once)
        int x = 0;
        do {
            x = 42;
        } while (false);
        assert x == 42 : "do-while with false condition executes once";
        
        // Nested do-while
        int outer = 0;
        int total = 0;
        do {
            int inner = 0;
            do {
                total++;
                inner++;
            } while (inner < 2);
            outer++;
        } while (outer < 3);
        assert total == 6 : "nested do-while loops (3*2)";
        
        // Do-while with break
        int i = 0;
        do {
            i++;
            if (i >= 10) {
                break;
            }
        } while (true);
        assert i == 10 : "do-while with break";
        
        // Do-while with continue
        int j = 0;
        int oddCount = 0;
        do {
            j++;
            if (j % 2 == 0) {
                continue;
            }
            oddCount++;
        } while (j < 10);
        assert oddCount == 5 : "do-while with continue (odd numbers 1..10)";
        
        System.out.println("Do-while loop tests passed.");
    }
}