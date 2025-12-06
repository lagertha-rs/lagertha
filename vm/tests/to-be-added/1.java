public class TTTes {
    class MyClass {}

    interface MyInterface {}

    class MyImpl implements MyInterface {}

    class MyRunnable implements Runnable {
        public void run() {}
    }

    class Parent {}

    class Child extends Parent {}

    class GrandChild extends Child {}

    interface BaseInterface {}

    interface SubInterface extends BaseInterface {}

    class SubInterfaceImpl implements SubInterface {}

    public static void main(String[] args) {
        testSameClass();
        testSuperclass();
        testInterfaces();
        testObject();
        testPrimitives();
        testArrays();
        testUnrelated();
        testNullThrows();

        System.out.println("All isAssignableFrom tests passed");
    }

    static void testSameClass() {
        // Same class is always assignable from itself
        assert String.class.isAssignableFrom(String.class) : "Same class failed";
        assert Object.class.isAssignableFrom(Object.class) : "Same Object failed";
        assert MyClass.class.isAssignableFrom(MyClass.class) : "Same MyClass failed";
    }

    static void testSuperclass() {
        // Superclass.isAssignableFrom(Subclass) = true
        assert Object.class.isAssignableFrom(String.class) : "Object from String failed";
        assert Number.class.isAssignableFrom(Integer.class) : "Number from Integer failed";
        assert Parent.class.isAssignableFrom(Child.class) : "Parent from Child failed";
        assert Parent.class.isAssignableFrom(GrandChild.class) : "Parent from GrandChild failed";

        // Subclass.isAssignableFrom(Superclass) = false
        assert !String.class.isAssignableFrom(Object.class) : "String from Object should be false";
        assert !Integer.class.isAssignableFrom(Number.class) : "Integer from Number should be false";
        assert !Child.class.isAssignableFrom(Parent.class) : "Child from Parent should be false";
    }

    static void testInterfaces() {
        // Interface.isAssignableFrom(ImplementingClass) = true
        assert Runnable.class.isAssignableFrom(MyRunnable.class) : "Runnable from MyRunnable failed";
        assert MyInterface.class.isAssignableFrom(MyImpl.class) : "MyInterface from MyImpl failed";

        // Interface.isAssignableFrom(SubInterface) = true
        assert BaseInterface.class.isAssignableFrom(SubInterface.class) : "BaseInterface from SubInterface failed";

        // Class implementing sub-interface is assignable to base interface
        assert BaseInterface.class.isAssignableFrom(SubInterfaceImpl.class) : "BaseInterface from SubInterfaceImpl failed";

        // Class.isAssignableFrom(Interface) = false (unless class is Object)
        assert !MyImpl.class.isAssignableFrom(MyInterface.class) : "MyImpl from MyInterface should be false";

        // Object.isAssignableFrom(Interface) = true
        assert Object.class.isAssignableFrom(Runnable.class) : "Object from Runnable failed";
        assert Object.class.isAssignableFrom(MyInterface.class) : "Object from MyInterface failed";
    }

    static void testObject() {
        // Object is assignable from any reference type
        assert Object.class.isAssignableFrom(String.class) : "Object from String failed";
        assert Object.class.isAssignableFrom(Integer.class) : "Object from Integer failed";
        assert Object.class.isAssignableFrom(Object[].class) : "Object from Object[] failed";
        assert Object.class.isAssignableFrom(int[].class) : "Object from int[] failed";

        // But not from primitives
        assert !Object.class.isAssignableFrom(int.class) : "Object from int should be false";
        assert !Object.class.isAssignableFrom(void.class) : "Object from void should be false";
    }

    static void testPrimitives() {
        // Primitives only assignable from themselves
        assert int.class.isAssignableFrom(int.class) : "int from int failed";
        assert long.class.isAssignableFrom(long.class) : "long from long failed";
        assert boolean.class.isAssignableFrom(boolean.class) : "boolean from boolean failed";
        assert void.class.isAssignableFrom(void.class) : "void from void failed";

        // Not from other primitives (no widening at Class level)
        assert !long.class.isAssignableFrom(int.class) : "long from int should be false";
        assert !double.class.isAssignableFrom(float.class) : "double from float should be false";
        assert !int.class.isAssignableFrom(long.class) : "int from long should be false";

        // Not from wrappers
        assert !int.class.isAssignableFrom(Integer.class) : "int from Integer should be false";
        assert !Integer.class.isAssignableFrom(int.class) : "Integer from int should be false";
    }

    static void testArrays() {
        // Same array type
        assert int[].class.isAssignableFrom(int[].class) : "int[] from int[] failed";
        assert Object[].class.isAssignableFrom(Object[].class) : "Object[] from Object[] failed";

        // Object[] is assignable from SubClass[]
        assert Object[].class.isAssignableFrom(String[].class) : "Object[] from String[] failed";
        assert Number[].class.isAssignableFrom(Integer[].class) : "Number[] from Integer[] failed";
        assert Parent[].class.isAssignableFrom(Child[].class) : "Parent[] from Child[] failed";

        // But not the reverse
        assert !String[].class.isAssignableFrom(Object[].class) : "String[] from Object[] should be false";

        // Primitive arrays not assignable to each other
        assert !int[].class.isAssignableFrom(long[].class) : "int[] from long[] should be false";
        assert !long[].class.isAssignableFrom(int[].class) : "long[] from int[] should be false";

        // Object[] not assignable from primitive arrays
        assert !Object[].class.isAssignableFrom(int[].class) : "Object[] from int[] should be false";

        // But Object is assignable from any array
        assert Object.class.isAssignableFrom(int[].class) : "Object from int[] failed";
        assert Object.class.isAssignableFrom(Object[].class) : "Object from Object[] failed";

        // Cloneable and Serializable from arrays
        assert Cloneable.class.isAssignableFrom(int[].class) : "Cloneable from int[] failed";
        assert Cloneable.class.isAssignableFrom(Object[].class) : "Cloneable from Object[] failed";
        assert java.io.Serializable.class.isAssignableFrom(int[].class) : "Serializable from int[] failed";

        // Multi-dimensional arrays
        assert Object[][].class.isAssignableFrom(String[][].class) : "Object[][] from String[][] failed";
        assert Object[].class.isAssignableFrom(String[][].class) : "Object[] from String[][] failed";
    }

    static void testUnrelated() {
        // Unrelated classes
        assert !String.class.isAssignableFrom(Integer.class) : "String from Integer should be false";
        assert !Integer.class.isAssignableFrom(String.class) : "Integer from String should be false";
        assert !MyClass.class.isAssignableFrom(MyImpl.class) : "MyClass from MyImpl should be false";

        // Unrelated interfaces
        assert !Runnable.class.isAssignableFrom(MyInterface.class) : "Runnable from MyInterface should be false";
    }

    static void testNullThrows() {
        boolean threw = false;
        try {
            Object.class.isAssignableFrom(null);
        } catch (NullPointerException e) {
            threw = true;
        }
        assert threw : "Should throw NullPointerException for null";
    }
}