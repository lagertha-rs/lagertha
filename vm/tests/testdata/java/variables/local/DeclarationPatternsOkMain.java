package variables.local.declaration_patterns;

public class DeclarationPatternsOkMain {
    public static void main(String[] args) {
        // Declaration without initialization, then assign
        int uninitialized;
        uninitialized = 99;
        assert uninitialized == 99 : "uninit.then.assign";

        // Multiple declarations in one statement
        int a, b, c;
        a = 1;
        b = 2;
        c = 3;
        assert a == 1 : "multi.decl.a";
        assert b == 2 : "multi.decl.b";
        assert c == 3 : "multi.decl.c";

        // Multiple declarations with partial initialization
        int x = 10, y, z = 30;
        y = 20;
        assert x == 10 : "partial.init.x";
        assert y == 20 : "partial.init.y";
        assert z == 30 : "partial.init.z";

        // Declaration with expression
        int sum = 5 + 3;
        assert sum == 8 : "expr.init";

        // Declaration with method call result
        int fromMethod = getFortyTwo();
        assert fromMethod == 42 : "method.init";

        // Declaration with ternary
        int ternary = true ? 100 : 200;
        assert ternary == 100 : "ternary.init";

        // Reassignment
        int val = 1;
        assert val == 1 : "reassign.before";
        val = 2;
        assert val == 2 : "reassign.after";
        val = 3;
        assert val == 3 : "reassign.again";

        // Multiple reassignments in sequence
        int seq = 0;
        seq = 1;
        seq = 2;
        seq = 3;
        assert seq == 3 : "seq.reassign";

        // Assignment chain
        int chainA, chainB, chainC;
        chainA = chainB = chainC = 42;
        assert chainA == 42 : "chain.a";
        assert chainB == 42 : "chain.b";
        assert chainC == 42 : "chain.c";

        System.out.println("All declaration pattern tests passed.");
    }

    static int getFortyTwo() {
        return 42;
    }
}
