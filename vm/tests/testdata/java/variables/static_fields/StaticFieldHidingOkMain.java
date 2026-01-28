package variables.static_fields.static_field_hiding;

public class StaticFieldHidingOkMain {
    public static void main(String[] args) {
        // Parent and child have same-named static field
        assert HidingStaticParent.value == 100 : "hide.parent.value";
        assert HidingStaticChild.value == 999 : "hide.child.value";

        // Modify child's doesn't affect parent's
        HidingStaticChild.value = 1000;
        assert HidingStaticParent.value == 100 : "hide.parent.unchanged";
        assert HidingStaticChild.value == 1000 : "hide.child.changed";

        // Reset
        HidingStaticChild.value = 999;

        // Access via instance reference
        HidingStaticParent parentRef = new HidingStaticChild();
        HidingStaticChild childRef = new HidingStaticChild();
        
        // Static field access is based on declared type, not runtime type
        assert parentRef.value == 100 : "hide.via.parentref";
        assert childRef.value == 999 : "hide.via.childref";

        System.out.println("Static field hiding test passed.");
    }
}

class HidingStaticParent {
    static int value = 100;
}

class HidingStaticChild extends HidingStaticParent {
    static int value = 999; // Hides parent's static field
}
