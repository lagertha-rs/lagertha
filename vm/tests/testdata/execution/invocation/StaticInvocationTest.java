// @test feature = "execution.invocation.static"
// @test description = "Verifies static method invocation on classes and interfaces."
// @test category = "success"

package execution.invocation;

public class StaticInvocationTest {
    public static void main(String[] args) {
        assert StaticMethods.classValue() == 42 : "static.class";
        assert StaticInterface.interfaceValue() == 999 : "static.interface";
    }
}

class StaticMethods {
    static int classValue() {
        return 42;
    }
}

interface StaticInterface {
    static int interfaceValue() {
        return 999;
    }
}
