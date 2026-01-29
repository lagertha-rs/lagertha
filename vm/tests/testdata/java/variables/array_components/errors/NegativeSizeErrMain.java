package variables.array_components.errors.negative_size;

public class NegativeSizeErrMain {
    public static void main(String[] args) {
        // This should throw NegativeArraySizeException
        int[] a = new int[-1];
        System.out.println("Should not print");
    }
}
