package control_flow.loops.while_loop;

public class WhileLoopOkMain {
    public static void main(String[] args) {
        // Basic while loop
        int count = 0;
        int sum = 0;
        while (count < 5) {
            sum += count;
            count++;
        }
        assert sum == 10 : "while loop sum 0..4 = 10";
        assert count == 5 : "while loop executes 5 times";
        
        // While with false condition (zero iterations)
        int zero = 0;
        while (zero > 0) {
            zero = -1; // never executed
        }
        assert zero == 0 : "while loop with false condition";
        
        // Nested while loops
        int outer = 0;
        int total = 0;
        while (outer < 3) {
            int inner = 0;
            while (inner < 2) {
                total++;
                inner++;
            }
            outer++;
        }
        assert total == 6 : "nested while loops (3*2)";
        
        // While with break
        int i = 0;
        while (true) {
            i++;
            if (i >= 10) {
                break;
            }
        }
        assert i == 10 : "while with break";
        
        // While with continue
        int j = 0;
        int oddCount = 0;
        while (j < 10) {
            j++;
            if (j % 2 == 0) {
                continue;
            }
            oddCount++;
        }
        assert oddCount == 5 : "while with continue (odd numbers 1..10)";
        
        System.out.println("While loop tests passed.");
    }
}