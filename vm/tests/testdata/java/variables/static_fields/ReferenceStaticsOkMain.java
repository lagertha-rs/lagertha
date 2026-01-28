package variables.static_fields.reference_statics;

public class ReferenceStaticsOkMain {
    public static void main(String[] args) {
        // Test initialized values
        assert ReferenceStatics.objectField != null : "static.ref.object.init";
        assert ReferenceStatics.stringField != null : "static.ref.string.init";
        assert ReferenceStatics.stringField == "static" : "static.ref.string.value";
        assert ReferenceStatics.intArrayField != null : "static.ref.intarr.init";
        assert ReferenceStatics.intArrayField.length == 5 : "static.ref.intarr.len";
        assert ReferenceStatics.objectArrayField != null : "static.ref.objarr.init";
        assert ReferenceStatics.objectArrayField.length == 3 : "static.ref.objarr.len";

        // Test null assignment
        Object savedObj = ReferenceStatics.objectField;
        ReferenceStatics.objectField = null;
        assert ReferenceStatics.objectField == null : "static.ref.object.null";
        ReferenceStatics.objectField = savedObj;

        String savedStr = ReferenceStatics.stringField;
        ReferenceStatics.stringField = null;
        assert ReferenceStatics.stringField == null : "static.ref.string.null";
        ReferenceStatics.stringField = savedStr;

        // Test reassignment
        ReferenceStatics.stringField = "modified";
        assert ReferenceStatics.stringField == "modified" : "static.ref.string.reassign";
        ReferenceStatics.stringField = "static";

        // Test array modification
        ReferenceStatics.intArrayField[0] = 999;
        assert ReferenceStatics.intArrayField[0] == 999 : "static.ref.arr.mod";
        ReferenceStatics.intArrayField[0] = 0;

        System.out.println("Reference static fields test passed.");
    }
}

class ReferenceStatics {
    static Object objectField = new Object();
    static String stringField = "static";
    static int[] intArrayField = new int[5];
    static Object[] objectArrayField = new Object[3];
}
