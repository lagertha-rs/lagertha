package inheritance.invoke_parent_setter;

public class InvokeParentSetterMain {
    static class Parent {
        private int parentId;

        public void setParentId(int parentId) {
            this.parentId = parentId;
        }
    }

    static class Child extends Parent {}

    public static void main(String[] args) {
        Child child = new Child();
        child.setParentId(224);
    }
}
