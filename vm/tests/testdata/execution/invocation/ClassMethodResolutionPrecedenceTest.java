// @test feature = "execution.invocation.virtual"
// @test description = "Prefers class and superclass declarations over same-signature interface defaults."
// @test category = "success"

package execution.invocation;

public class ClassMethodResolutionPrecedenceTest {
    public static void main(String[] args) {
        DirectClassDeclaration direct = new DirectClassDeclaration();
        assert direct.value() == 10 : "class.before.interface";

        InheritedClassDeclaration inherited = new InheritedClassDeclaration();
        assert inherited.value() == 20 : "superclass.before.interface";
    }
}

interface ResolutionPrecedenceInterface {
    default int value() {
        return 30;
    }
}

class DirectClassDeclaration implements ResolutionPrecedenceInterface {
    @Override
    public int value() {
        return 10;
    }
}

class ConcreteMethodSuperclass {
    public int value() {
        return 20;
    }
}

class InheritedClassDeclaration extends ConcreteMethodSuperclass implements ResolutionPrecedenceInterface {
}
