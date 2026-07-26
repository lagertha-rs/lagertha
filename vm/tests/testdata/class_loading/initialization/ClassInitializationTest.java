// @test feature = "class-loading.initialization"
// @test description = "Verifies active initialization triggers, superclass and textual order, interfaces, and one-time execution."
// @test category = "success"

package classloading.initialization;

public class ClassInitializationTest {
    public static void main(String[] args) {
        assert InitializationTrace.sequence == 0 : "initialization.trace.initial";

        assert InitializationChild.childMarker == 2 : "initialization.static.read";
        assert InitializationTrace.sequence == 12 : "initialization.superclass.order";
        assert InitializationChild.childMarker == 2 : "initialization.static.read.repeat";
        assert InitializationTrace.sequence == 12 : "initialization.once";

        WriteTrigger.value = 8;
        assert WriteTrigger.value == 8 : "initialization.static.write";
        assert InitializationTrace.sequence == 123 : "initialization.static.write.trigger";

        new NewTrigger();
        assert InitializationTrace.sequence == 1234 : "initialization.new.trigger";

        assert BlockOrder.sequence == 12 : "initialization.block.order";
        assert BlockOrder.observedFirst == 1 : "initialization.field.order";

        assert DirectInterface.MARKER == 5 : "initialization.interface.read";
        assert InitializationTrace.sequence == 12345 : "initialization.interface.trigger";
    }
}

class InitializationTrace {
    static int sequence;

    static int record(int marker) {
        sequence = sequence * 10 + marker;
        return marker;
    }
}

class InitializationParent {
    static int parentMarker = InitializationTrace.record(1);
}

class InitializationChild extends InitializationParent {
    static int childMarker = InitializationTrace.record(2);
}

class WriteTrigger {
    static int marker = InitializationTrace.record(3);
    static int value;
}

class NewTrigger {
    static int marker = InitializationTrace.record(4);
}

class BlockOrder {
    static int sequence;

    static {
        sequence = 1;
    }

    static int observedFirst = sequence;

    static {
        sequence = sequence * 10 + 2;
    }
}

interface DirectInterface {
    int MARKER = InitializationTrace.record(5);
}
