package variables.defaults.inherited_field_defaults;

public class InheritedFieldDefaultsOkMain {
    public static void main(String[] args) {
        // Child class doesn't initialize parent's fields explicitly
        ChildClass child = new ChildClass();

        // Inherited fields should have defaults
        assert child.parentInt == 0 : "inherit.int.default";
        assert child.parentLong == 0L : "inherit.long.default";
        assert child.parentBool == false : "inherit.bool.default";
        assert child.parentObject == null : "inherit.object.default";

        // Child's own fields should also have defaults
        assert child.childInt == 0 : "child.int.default";
        assert child.childObject == null : "child.object.default";

        // Multiple levels of inheritance
        GrandchildClass grandchild = new GrandchildClass();
        assert grandchild.parentInt == 0 : "grandchild.parent.int.default";
        assert grandchild.childInt == 0 : "grandchild.child.int.default";
        assert grandchild.grandchildInt == 0 : "grandchild.own.int.default";

        System.out.println("Inherited field defaults test passed.");
    }
}

class ParentClass {
    int parentInt;
    long parentLong;
    boolean parentBool;
    Object parentObject;
}

class ChildClass extends ParentClass {
    int childInt;
    Object childObject;
}

class GrandchildClass extends ChildClass {
    int grandchildInt;
}
