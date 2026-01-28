package variables.defaults.nested_class_defaults;

public class NestedClassDefaultsOkMain {
    public static void main(String[] args) {
        // Static nested class
        StaticNestedClass nested = new StaticNestedClass();
        assert nested.nestedInt == 0 : "nested.static.int.default";
        assert nested.nestedObject == null : "nested.static.object.default";

        // Inner class (requires outer instance)
        NestedClassDefaultsOkMain outer = new NestedClassDefaultsOkMain();
        InnerClass inner = outer.new InnerClass();
        assert inner.innerInt == 0 : "nested.inner.int.default";
        assert inner.innerObject == null : "nested.inner.object.default";

        System.out.println("Nested class defaults test passed.");
    }

    static class StaticNestedClass {
        int nestedInt;
        Object nestedObject;
    }

    class InnerClass {
        int innerInt;
        Object innerObject;
    }
}
