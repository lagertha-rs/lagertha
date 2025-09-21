package inheritance.constructor_chain;

public class ConstructorChainMain {
    static class Parent {
        final int id;
        Parent(int id) { this.id = id; }
    }

    static class Child extends Parent {
        Child(int id) { super(id); }
    }

    public static void main(String[] args) {
        Child c = new Child(42);
        int v = c.id;
    }
}
