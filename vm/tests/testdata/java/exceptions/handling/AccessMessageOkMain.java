package exceptions.handling.access_message;

public class AccessMessageOkMain {
    public static void main(String[] args) {
        try {
            throw new IllegalArgumentException("Test message");
        } catch (Exception e) {
            System.out.println(e.getMessage());
        }
    }
}