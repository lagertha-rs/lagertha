// @test feature = "execution.invocation.virtual"
// @test description = "Verifies runtime override selection and abstract superclass dispatch."
// @test category = "success"

package execution.invocation;

public class VirtualDispatchTest {
    public static void main(String[] args) {
        VirtualParent parent = new VirtualChild();
        assert parent.value() == 200 : "virtual.override";

        VirtualAbstract abstractReceiver = new VirtualConcrete();
        assert abstractReceiver.abstractValue() == 42 : "virtual.abstract.implementation";
        assert abstractReceiver.concreteValue() == 100 : "virtual.inherited.concrete";
    }
}

class VirtualParent {
    int value() {
        return 100;
    }
}

class VirtualChild extends VirtualParent {
    @Override
    int value() {
        return 200;
    }
}

abstract class VirtualAbstract {
    abstract int abstractValue();

    int concreteValue() {
        return 100;
    }
}

class VirtualConcrete extends VirtualAbstract {
    @Override
    int abstractValue() {
        return 42;
    }
}
