package exceptions.exception_in_catch;

public class ExceptionInCatchBlockShouldWorkOkMain {
    public static void main(String[] args) {
        try {
            throw new NullPointerException("Original");
        } catch (Throwable e) {
            System.out.println("In catch block");
            try {
                throw new IllegalArgumentException("New exception");
            } catch (IllegalArgumentException inner) {
                System.out.println("Caught inner exception");
            }
        }
    }
}