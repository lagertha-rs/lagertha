package exceptions.handling.simple_catch;

public class SimpleCatchOkMain {
    public static void main(String[] args) {
        try {
            throw new IllegalArgumentException("Test");
        } catch (Throwable e) {
            System.out.println("Caught exception");
        }
    }
}