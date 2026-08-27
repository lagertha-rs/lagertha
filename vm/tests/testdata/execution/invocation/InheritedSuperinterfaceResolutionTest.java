// @test feature = "execution.invocation.virtual"
// @test description = "Resolves a default method inherited through a child interface."
// @test category = "success"

package execution.invocation;

public class InheritedSuperinterfaceResolutionTest {
    public static void main(String[] args) {
        InheritedDefaultImplementation implementation = new InheritedDefaultImplementation();
        assert implementation.value() == 30 : "inherited.superinterface.default";
    }
}

interface InheritedDefaultParent {
    default int value() {
        return 30;
    }
}

interface InheritedDefaultChild extends InheritedDefaultParent {
}

class InheritedDefaultImplementation implements InheritedDefaultChild {
}
