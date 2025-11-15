package exceptions.catch_access_message;

public class CatchExceptionAccessMessageShouldWorkOkMain {
    public static void main(String[] args) {
        try {
            throw new IllegalArgumentException("Test message");
        } catch (Exception e) {
            System.out.println(e.getMessage());
        }
    }
}