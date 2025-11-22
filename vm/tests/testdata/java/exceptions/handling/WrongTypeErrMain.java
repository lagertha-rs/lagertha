package exceptions.handling.wrong_type;

public class WrongTypeErrMain {
    public static void main(String[] args) {
        try {
            throw new NullPointerException("NPE");
        } catch (IllegalArgumentException e) {
            System.out.println("Caught IAE");
        }
    }
}