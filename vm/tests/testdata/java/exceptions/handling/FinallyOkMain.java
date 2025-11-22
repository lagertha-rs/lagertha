package exceptions.handling.finally_block;

public class FinallyOkMain {
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