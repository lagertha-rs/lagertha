package inheritance.call_parent_method;

public class CallParentMethodMain {
    static class Parent {
        private final int parentId = 42;

        public int getParentId() {
            return parentId;
        }
    }

    static class Child extends Parent {}

    public static void main(String[] args) {
        Child child = new Child();
        var parentId = child.getParentId();
    }
}
