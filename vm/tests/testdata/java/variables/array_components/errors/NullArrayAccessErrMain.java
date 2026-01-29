package variables.array_components.errors.null_array_access;

public class NullArrayAccessErrMain {
    public static void main(String[] args) {
        int[] arr = null;
        // This should throw NullPointerException
        int x = arr[0];
        System.out.println("Should not print: " + x);
    }
}
