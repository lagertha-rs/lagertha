// @test feature = "execution.invocation.virtual"
// @test description = "Resolves an invokevirtual class method reference through a superinterface default method."
// @test category = "success"

package execution.invocation;

public class ClassMethodInterfaceResolutionTest {
    public static void main(String[] args) {
        DefaultMethodImplementation implementation = new DefaultMethodImplementation();
        assert implementation.value() == 42 : "interface.default.resolution";
    }
}

interface DefaultMethodInterface {
    default int value() {
        return 42;
    }
}

class DefaultMethodImplementation implements DefaultMethodInterface {
}
