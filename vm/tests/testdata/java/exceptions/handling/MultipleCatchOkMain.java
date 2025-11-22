package exceptions.handling.multiple_catch;

public class MultipleCatchOkMain {
    public static void main(String[] args) {
        try {
            throw new NullPointerException("NPE");
        } catch (IllegalArgumentException e) {
            System.out.println("Caught IllegalArgumentException");
        } catch (NullPointerException e) {
            System.out.println("Caught NullPointerException");
        } catch (Throwable e) {
            System.out.println("Caught other exception");
        }
    }
}