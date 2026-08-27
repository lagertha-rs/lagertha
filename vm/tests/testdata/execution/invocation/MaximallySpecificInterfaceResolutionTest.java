// @test feature = "execution.invocation.virtual"
// @test description = "Selects a child-interface default over its parent-interface default."
// @test category = "success"

package execution.invocation;

public class MaximallySpecificInterfaceResolutionTest {
    public static void main(String[] args) {
        ChildDefaultImplementation childDefault = new ChildDefaultImplementation();
        assert childDefault.value() == 20 : "child.interface.default";
    }
}

interface ParentDefaultInterface {
    default int value() {
        return 10;
    }
}

interface ChildDefaultInterface extends ParentDefaultInterface {
    @Override
    default int value() {
        return 20;
    }
}

class ChildDefaultImplementation implements ChildDefaultInterface {
}
