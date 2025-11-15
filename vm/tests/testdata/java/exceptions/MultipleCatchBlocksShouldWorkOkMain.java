package exceptions.multiple_catches;

public class MultipleCatchBlocksShouldWorkOkMain {
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