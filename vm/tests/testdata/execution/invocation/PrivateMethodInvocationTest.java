// @test feature = "execution.invocation.virtual"
// @test description = "Verifies private method selection ignores same-signature subclass methods."
// @test category = "success"

package execution.invocation;

public class PrivateMethodInvocationTest {
    public static void main(String[] args) {
        PrivateMethodChild child = new PrivateMethodChild();

        assert child.value() == 2 : "private.child.independent";
        assert child.callPrivate() == 1 : "private.resolved.declaration";
    }
}

class PrivateMethodParent {
    private int value() {
        return 1;
    }

    int callPrivate() {
        return value();
    }
}

class PrivateMethodChild extends PrivateMethodParent {
    int value() {
        return 2;
    }
}
