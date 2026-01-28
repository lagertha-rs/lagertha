package variables.instance.inheritance;

public class InheritanceOkMain {
    public static void main(String[] args) {
        Child child = new Child();

        // Child sees parent's field
        assert child.parentValue == 100 : "inherit.parent.field";

        // Child can modify parent's field
        child.parentValue = 200;
        assert child.parentValue == 200 : "inherit.parent.mod";

        // Child's own field
        assert child.childValue == 50 : "inherit.child.field";

        // Access via parent reference
        Parent asParent = child;
        assert asParent.parentValue == 200 : "inherit.via.parent";

        // Method in child that uses both
        assert child.getSum() == 250 : "inherit.sum";

        // Different child instances independent
        Child child2 = new Child();
        assert child2.parentValue == 100 : "inherit.child2.parent";
        assert child.parentValue == 200 : "inherit.child1.still";

        System.out.println("Inheritance test passed.");
    }
}

class Parent {
    int parentValue = 100;
}

class Child extends Parent {
    int childValue = 50;

    int getSum() {
        return parentValue + childValue;
    }
}
