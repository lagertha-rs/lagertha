package variables.local.reference_types;

public class ReferenceTypesOkMain {
    public static void main(String[] args) {
        // Object reference
        Object objVar = new Object();
        assert objVar != null : "object.init";
        
        // String reference
        String strVar = "hello";
        assert strVar != null : "string.init";
        assert strVar == "hello" : "string.value";
        
        // Array references
        int[] intArrVar = new int[5];
        assert intArrVar != null : "intarr.init";
        assert intArrVar.length == 5 : "intarr.length";
        
        Object[] objArrVar = new Object[3];
        assert objArrVar != null : "objarr.init";
        assert objArrVar.length == 3 : "objarr.length";
        
        // Custom class reference
        Helper helper = new Helper();
        assert helper != null : "custom.init";

        // Null assignment
        Object obj = new Object();
        assert obj != null : "null.before";
        obj = null;
        assert obj == null : "null.after";

        String str = "hello";
        str = null;
        assert str == null : "null.string";

        int[] arr = new int[5];
        arr = null;
        assert arr == null : "null.array";

        Object nullInit = null;
        assert nullInit == null : "null.init";

        Object reassigned = null;
        reassigned = new Object();
        assert reassigned != null : "null.reassign";

        System.out.println("All reference type local variable tests passed.");
    }

    static class Helper {
        int value = 0;
    }
}
