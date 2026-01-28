package variables.local.increment_decrement;

public class IncrementDecrementOkMain {
    public static void main(String[] args) {
        int val;

        // Pre-increment
        val = 5;
        assert ++val == 6 : "pre.inc.result";
        assert val == 6 : "pre.inc.after";

        // Post-increment
        val = 5;
        assert val++ == 5 : "post.inc.result";
        assert val == 6 : "post.inc.after";

        // Pre-decrement
        val = 5;
        assert --val == 4 : "pre.dec.result";
        assert val == 4 : "pre.dec.after";

        // Post-decrement
        val = 5;
        assert val-- == 5 : "post.dec.result";
        assert val == 4 : "post.dec.after";

        // Increment on byte
        byte byteVal = 126;
        byteVal++;
        assert byteVal == 127 : "inc.byte";
        byteVal++;
        assert byteVal == -128 : "inc.byte.wrap";

        // Decrement on long
        long longVal = Long.MIN_VALUE;
        longVal--;
        assert longVal == Long.MAX_VALUE : "dec.long.wrap";

        // Multiple increments in expression
        val = 0;
        int result = val++ + ++val;
        // val++ returns 0, val becomes 1
        // ++val makes val 2, returns 2
        // result = 0 + 2 = 2
        assert result == 2 : "multi.inc.expr";
        assert val == 2 : "multi.inc.val";

        System.out.println("All increment/decrement tests passed.");
    }
}
