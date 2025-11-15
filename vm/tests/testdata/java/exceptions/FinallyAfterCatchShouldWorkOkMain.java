package exceptions.finally_after_catch;

public class FinallyAfterCatchShouldWorkOkMain {
    public static void main(String[] args) {
        try {
            throw new NullPointerException("Test");
        } catch (Throwable e) {
            System.out.println("In catch block");
        } finally {
            System.out.println("In finally block");
        }
    }
}