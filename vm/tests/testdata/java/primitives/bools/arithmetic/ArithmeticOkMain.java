package primitives.bools.arithmetic.comprehensive;

public class ArithmeticOkMain {
    public static void main(String[] args) {
        // Basic operations (in Java, boolean is not numeric, so limited operations)

        // Logical operators
        assert (true && true) : "and.both.true";
        assert !(true && false) : "and.mixed";
        assert !(false && false) : "and.both.false";

        assert (true || true) : "or.both.true";
        assert (true || false) : "or.mixed";
        assert !(false || false) : "or.both.false";

        assert !true == false : "not.true";
        assert !false == true : "not.false";

        // XOR-like behavior (!=)
        assert (true != false) : "xor.different";
        assert !(true != true) : "xor.same";

        // Equality
        assert (true == true) : "eq.true.true";
        assert (false == false) : "eq.false.false";
        assert (true != false) : "ne.true.false";

        // Short-circuit evaluation
        boolean sideEffect = false;
        boolean result = false && (sideEffect = true);
        assert !sideEffect : "shortcircuit.and";

        sideEffect = false;
        result = true || (sideEffect = true);
        assert !sideEffect : "shortcircuit.or";

        // Ternary operator with boolean
        assert (true ? 1 : 0) == 1 : "ternary.true";
        assert (false ? 1 : 0) == 0 : "ternary.false";

        // Boolean as method return
        assert Boolean.TRUE : "Boolean.TRUE";
        assert !Boolean.FALSE : "Boolean.FALSE";

        System.out.println("All boolean assertions passed.");
    }
}