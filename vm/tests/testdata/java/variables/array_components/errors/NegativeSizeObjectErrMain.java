package variables.array_components.errors.negative_size_object;

public class NegativeSizeObjectErrMain {
    public static void main(String[] args) {
        // This should throw NegativeArraySizeException
        String[] a = new String[-1];
        System.out.println("Should not print");
    }
}
