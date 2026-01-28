package variables.instance.reference_fields;

public class ReferenceFieldsOkMain {
    public static void main(String[] args) {
        ReferenceFieldHolder holder = new ReferenceFieldHolder();

        // Test initialized values
        assert holder.objectField != null : "ref.object.init";
        assert holder.stringField != null : "ref.string.init";
        assert holder.stringField == "hello" : "ref.string.value";
        assert holder.intArrayField != null : "ref.intarr.init";
        assert holder.intArrayField.length == 5 : "ref.intarr.len";
        assert holder.objectArrayField != null : "ref.objarr.init";
        assert holder.objectArrayField.length == 3 : "ref.objarr.len";

        // Test null assignment
        holder.objectField = null;
        assert holder.objectField == null : "ref.object.null";

        holder.stringField = null;
        assert holder.stringField == null : "ref.string.null";

        // Test reassignment
        holder.objectField = new Object();
        assert holder.objectField != null : "ref.object.reassign";

        holder.stringField = "world";
        assert holder.stringField == "world" : "ref.string.reassign";

        // Test array reassignment
        int[] newArr = new int[10];
        holder.intArrayField = newArr;
        assert holder.intArrayField.length == 10 : "ref.intarr.reassign";

        // Self-referencing field
        SelfRef selfRef = new SelfRef();
        assert selfRef.next == null : "selfref.init.null";
        selfRef.next = selfRef;
        assert selfRef.next == selfRef : "selfref.self";
        selfRef.next = new SelfRef();
        assert selfRef.next != selfRef : "selfref.other";

        System.out.println("Reference fields test passed.");
    }
}

class ReferenceFieldHolder {
    Object objectField = new Object();
    String stringField = "hello";
    int[] intArrayField = new int[5];
    Object[] objectArrayField = new Object[3];
}

class SelfRef {
    SelfRef next;
    int data;
}
