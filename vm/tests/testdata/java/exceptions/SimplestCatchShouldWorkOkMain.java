package exceptions.simplest_catch_block;

public class SimplestCatchShouldWorkOkMain {
    public static void main(String[] args) {
        try {
            throw new IllegalArgumentException("Test");
        } catch (Throwable e) {
            System.out.println("Caught exception");
        }
    }
}