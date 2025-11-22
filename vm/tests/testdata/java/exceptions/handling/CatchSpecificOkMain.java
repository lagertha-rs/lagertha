package exceptions.handling.catch_specific;

public class CatchSpecificOkMain {
    public static void main(String[] args) {
        try {
            throw new NullPointerException("NPE test");
        } catch (NullPointerException e) {
            System.out.println("Caught NullPointerException");
        }
    }
}