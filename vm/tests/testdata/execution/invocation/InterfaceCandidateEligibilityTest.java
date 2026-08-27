// @test feature = "execution.invocation.virtual"
// @test description = "Excludes private and static interface methods while resolving an eligible abstract interface method."
// @test category = "success"

package execution.invocation;

public class InterfaceCandidateEligibilityTest {
    public static void main(String[] args) {
        InterfaceCandidateOwner receiver = new InterfaceCandidateImplementation();
        assert receiver.value() == 50 : "eligible.interface.candidate";
    }
}

interface AbstractMethodCandidate {
    int value();
}

interface PrivateMethodCandidate {
    private int value() {
        return 10;
    }
}

interface StaticMethodCandidate {
    static int value() {
        return 20;
    }
}

abstract class InterfaceCandidateOwner
        implements AbstractMethodCandidate, PrivateMethodCandidate, StaticMethodCandidate {
}

class InterfaceCandidateImplementation extends InterfaceCandidateOwner {
    @Override
    public int value() {
        return 50;
    }
}
