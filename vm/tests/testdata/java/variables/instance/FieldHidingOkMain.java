package variables.instance.field_hiding;

public class FieldHidingOkMain {
    public static void main(String[] args) {
        HidingChild hChild = new HidingChild();

        // Child's field shadows parent's
        assert hChild.value == 999 : "hide.child.value";

        // Access parent's via cast
        HidingParent asParent = hChild;
        assert asParent.value == 100 : "hide.parent.via.cast";

        // Method in child sees child's field
        assert hChild.getChildValue() == 999 : "hide.child.method";

        // Inherited method from parent would see parent's field (if existed)
        // But we can test explicit access
        assert hChild.getParentValueExplicit() == 100 : "hide.parent.explicit";

        System.out.println("Field hiding test passed.");
    }
}

class HidingParent {
    int value = 100;
}

class HidingChild extends HidingParent {
    int value = 999; // Hides parent's field

    int getChildValue() {
        return this.value;
    }

    int getParentValueExplicit() {
        return ((HidingParent) this).value;
    }
}
