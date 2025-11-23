package exceptions.handling.constructor_exception;

public class ConstructorExceptionOkMain {
    static class Foo {
        Foo() {
            throw new IllegalArgumentException("Exception from constructor");
        }
    }
    public static void main(String[] args) {
        try {
            new Foo();
        }  catch (Exception e) {
            System.out.println("Caught!");
            e.printStackTrace();
        }
    }
}