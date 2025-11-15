package exceptions.catch_specific;

public class CatchSpecificExceptionShouldWorkOkMain {
    public static void main(String[] args) {
        try {
            throw new NullPointerException("NPE test");
        } catch (NullPointerException e) {
            System.out.println("Caught NullPointerException");
        }
    }
}