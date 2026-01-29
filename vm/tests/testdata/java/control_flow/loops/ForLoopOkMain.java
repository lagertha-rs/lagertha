package control_flow.loops.for_loop;

public class ForLoopOkMain {
    public static void main(String[] args) {
        // Basic for loop
        int sum = 0;
        for (int i = 0; i < 5; i++) {
            sum += i;
        }
        assert sum == 10 : "for loop sum 0..4 = 10";
        
        // For loop with empty body
        int count = 0;
        for (int i = 0; i < 5; i++, count++) {
            // empty
        }
        assert count == 5 : "for loop with empty body";
        
        // Nested for loops
        int total = 0;
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 2; j++) {
                total++;
            }
        }
        assert total == 6 : "nested for loops (3*2)";
        
        // For loop with break
        int iterations = 0;
        for (int i = 0; i < 100; i++) {
            iterations++;
            if (i >= 9) {
                break;
            }
        }
        assert iterations == 10 : "for loop with break";
        
        // For loop with continue
        int oddCount = 0;
        for (int i = 1; i <= 10; i++) {
            if (i % 2 == 0) {
                continue;
            }
            oddCount++;
        }
        assert oddCount == 5 : "for loop with continue (odd numbers 1..10)";
        
        // For loop with multiple initialization and update
        int x, y;
        for (x = 0, y = 10; x < y; x++, y--) {
            // ensure loop terminates
        }
        assert x == 5 && y == 5 : "for loop with multiple vars";
        
        // For loop without initialization (outside)
        int k = 0;
        sum = 0;
        for (; k < 5; k++) {
            sum += k;
        }
        assert sum == 10 : "for loop without init";
        
        // For loop without increment (in body)
        int m = 0;
        for (int n = 0; n < 5;) {
            m++;
            n++;
        }
        assert m == 5 : "for loop without increment in header";
        
        // Infinite for loop with break
        int infinite = 0;
        for (;;) {
            infinite++;
            if (infinite >= 7) {
                break;
            }
        }
        assert infinite == 7 : "infinite for loop with break";
        
        System.out.println("For loop tests passed.");
    }
}