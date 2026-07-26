// @test feature = "execution.control-flow.conditional-branches"
// @test description = "Verifies compiled boolean branches, short-circuiting, and conditional selection."
// @test category = "success"

package execution.controlflow.conditional;

public class BooleanBranchTest {
    public static void main(String[] args) {
        boolean trueValue = true;
        boolean falseValue = false;

        assert trueValue && trueValue : "and.both.true";
        assert !(trueValue && falseValue) : "and.mixed";
        assert trueValue || falseValue : "or.mixed";
        assert !(falseValue || falseValue) : "or.both.false";
        assert !falseValue : "not.false";
        assert trueValue != falseValue : "ne.different";
        assert trueValue == trueValue : "eq.same";

        boolean sideEffect = false;
        boolean result = falseValue && (sideEffect = true);
        assert !result : "shortcircuit.and.result";
        assert !sideEffect : "shortcircuit.and.effect";

        sideEffect = false;
        result = trueValue || (sideEffect = true);
        assert result : "shortcircuit.or.result";
        assert !sideEffect : "shortcircuit.or.effect";

        int selected = trueValue ? 1 : 0;
        assert selected == 1 : "conditional.true";
        selected = falseValue ? 1 : 0;
        assert selected == 0 : "conditional.false";
    }
}
