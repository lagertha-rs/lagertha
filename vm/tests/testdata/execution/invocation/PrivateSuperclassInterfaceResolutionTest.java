// @test feature = "execution.invocation.virtual"
// @test description = "Resolves a private superclass method before an interface default, then enforces access control."
// @test category = "error"

package execution.invocation;

public class PrivateSuperclassInterfaceResolutionTest {
    public static void main(String[] args) {
        PrivateSuperclassImplementation implementation = new PrivateSuperclassImplementation();
        implementation.value();
    }
}

interface PrivateResolutionInterface {
    default int value() {
        return 30;
    }
}

class PrivateMethodSuperclass {
    private int value() {
        return 40;
    }
}

class PrivateSuperclassImplementation extends PrivateMethodSuperclass implements PrivateResolutionInterface {
}
