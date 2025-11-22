package exceptions.propagation.uncaught;

public class UncaughtErrMain {
    static void throwException() {
        throw new NullPointerException("Exception outside try");
    }

    public static void main(String[] args) {
        try {
            System.out.println("In try");
        } catch (Throwable e) {
            System.out.println("Caught");
        }
        throwException();
    }
}