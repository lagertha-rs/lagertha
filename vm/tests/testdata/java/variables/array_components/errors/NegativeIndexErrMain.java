package variables.array_components.errors.negative_index;

public class NegativeIndexErrMain {
    public static void main(String[] args) {
        int[] arr = new int[5];
        // This should throw ArrayIndexOutOfBoundsException
        int x = arr[-1];
        System.out.println("Should not print: " + x);
    }
}
