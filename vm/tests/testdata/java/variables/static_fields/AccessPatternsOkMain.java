package variables.static_fields.access_patterns;

public class AccessPatternsOkMain {
    public static void main(String[] args) {
        // Access via class name
        assert AccessPatternStatics.value == 100 : "access.classname";

        // Access via instance (discouraged but valid)
        AccessPatternStatics instance = new AccessPatternStatics();
        assert instance.value == 100 : "access.instance";

        // Modify via class name
        AccessPatternStatics.value = 200;
        assert AccessPatternStatics.value == 200 : "access.classname.mod";

        // Modification visible via instance
        assert instance.value == 200 : "access.instance.sees.mod";

        // Modify via instance
        instance.value = 300;
        assert AccessPatternStatics.value == 300 : "access.classname.sees.instmod";

        // Multiple instances see same static
        AccessPatternStatics instance2 = new AccessPatternStatics();
        assert instance2.value == 300 : "access.instance2.sees";

        // Access without qualifier (within same class)
        assert AccessPatternStatics.testUnqualifiedAccess() == 300 : "access.unqualified";

        // Reset
        AccessPatternStatics.value = 100;

        System.out.println("Access patterns test passed.");
    }
}

class AccessPatternStatics {
    static int value = 100;

    static int testUnqualifiedAccess() {
        return value; // unqualified access within same class
    }
}
