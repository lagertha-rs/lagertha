// @test feature = "execution.invocation.interface"
// @test description = "Verifies implementing-class and default-method dispatch through an interface."
// @test category = "success"

package execution.invocation;

public class InterfaceInvocationTest {
    public static void main(String[] args) {
        DispatchInterface receiver = new DispatchImplementation();
        assert receiver.value() == 42 : "interface.implementation";
        assert receiver.defaultValue() == 100 : "interface.default";
    }
}

interface DispatchInterface {
    int value();

    default int defaultValue() {
        return 100;
    }
}

class DispatchImplementation implements DispatchInterface {
    @Override
    public int value() {
        return 42;
    }
}
