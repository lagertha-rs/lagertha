package exceptions.handling.catch_without_throw;

public class CatchWithoutThrowOkMain {
    public static void main(String[] args) {
        try {
            System.out.println("In try block");
        } catch (Throwable e) {
            System.out.println("Caught exception");
        }
        System.out.println("After try-catch");
    }
}