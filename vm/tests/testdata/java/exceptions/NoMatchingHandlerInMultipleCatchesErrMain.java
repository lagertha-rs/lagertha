package exceptions.no_matching_handler;

public class NoMatchingHandlerInMultipleCatchesErrMain {
    public static void main(String[] args) {
        try {
            throw new RuntimeException("Runtime error");
        } catch (IllegalArgumentException e) {
            System.out.println("Caught IAE");
        } catch (NullPointerException e) {
            System.out.println("Caught NPE");
        }
    }
}