package variables.static_fields.static_blocks;

public class StaticBlocksOkMain {
    public static void main(String[] args) {
        // Static block should have executed
        assert StaticBlockHolder.initialized == true : "staticblock.init";
        assert StaticBlockHolder.value == 42 : "staticblock.value";
        assert StaticBlockHolder.computedInBlock == 100 : "staticblock.computed";
        
        // Multiple static blocks execute in order
        assert MultiBlockHolder.order == 3 : "multiblock.order";
        // String concatenation uses invokedynamic, so check individual chars
        assert MultiBlockHolder.traceA == 'A' : "multiblock.traceA";
        assert MultiBlockHolder.traceB == 'B' : "multiblock.traceB";
        assert MultiBlockHolder.traceC == 'C' : "multiblock.traceC";

        System.out.println("Static blocks test passed.");
    }
}

class StaticBlockHolder {
    static boolean initialized;
    static int value;
    static int computedInBlock;

    static {
        initialized = true;
        value = 42;
        computedInBlock = 50 + 50;
    }
}

class MultiBlockHolder {
    static int order = 0;
    static char traceA;
    static char traceB;
    static char traceC;

    static {
        order++;
        traceA = 'A';
    }

    static {
        order++;
        traceB = 'B';
    }

    static {
        order++;
        traceC = 'C';
    }
}
