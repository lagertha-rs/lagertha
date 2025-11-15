package exceptions.exception_in_catch_propagates;

public class ExceptionInCatchBlockPropagatesErrMain {
    public static void main(String[] args) {
        try {
            throw new NullPointerException("Original");
        } catch (Throwable e) {
            throw new IllegalArgumentException("New exception in catch");
        }
    }
}