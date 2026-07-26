// @test feature = "execution.invocation.special"
// @test description = "Verifies that a super call selects the superclass method implementation."
// @test category = "success"

package execution.invocation;

public class SpecialInvocationTest {
    public static void main(String[] args) {
        SpecialChild child = new SpecialChild();
        assert child.value() == 200 : "virtual.override";
        assert child.parentValue() == 100 : "special.super";
    }
}

class SpecialParent {
    int value() {
        return 100;
    }
}

class SpecialChild extends SpecialParent {
    @Override
    int value() {
        return 200;
    }

    int parentValue() {
        return super.value();
    }
}
