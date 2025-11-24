package exceptions.propagation.constructor_exception;

public class ConstructorExceptionErrMain {
    static class Foo {
        Foo() {
            throw new IllegalArgumentException("Exception from constructor");
        }
    }
    public static void main(String[] args) {
        new Foo();
    }
}