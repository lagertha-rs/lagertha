package exceptions.wrong_type;

public class WrongExceptionTypeShouldFailErrMain {
    public static void main(String[] args) {
        try {
            throw new NullPointerException("NPE");
        } catch (IllegalArgumentException e) {
            System.out.println("Caught IAE");
        }
    }
}