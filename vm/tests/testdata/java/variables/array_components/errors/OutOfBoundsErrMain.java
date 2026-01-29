package variables.array_components.errors.out_of_bounds;

public class OutOfBoundsErrMain {
    public static void main(String[] args) {
        int[] arr = new int[5];
        // Array has indices 0-4, so index 5 is out of bounds
        // This should throw ArrayIndexOutOfBoundsException
        int x = arr[5];
        System.out.println("Should not print: " + x);
    }
}
