package variables.static_fields.static_inheritance;

public class StaticInheritanceOkMain {
    public static void main(String[] args) {
        // Child class can access parent's static field
        assert StaticChild.parentStatic == 100 : "inherit.parent.static";

        // Modify via child class name
        StaticChild.parentStatic = 200;
        assert StaticParent.parentStatic == 200 : "inherit.parent.sees.childmod";

        // Reset
        StaticParent.parentStatic = 100;

        // Child's own static
        assert StaticChild.childStatic == 50 : "inherit.child.static";

        System.out.println("Static inheritance test passed.");
    }
}

class StaticParent {
    static int parentStatic = 100;
}

class StaticChild extends StaticParent {
    static int childStatic = 50;
}
