package exceptions.outside_try_catch;

public class ExceptionOutsideTryCatchErrMain {
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