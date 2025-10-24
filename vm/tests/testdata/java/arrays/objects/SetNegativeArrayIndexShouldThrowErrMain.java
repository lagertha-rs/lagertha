package arrays.objects.set_negative_array_index_should_throw_err;

public class SetNegativeArrayIndexShouldThrowErrMain {
    public static void main(String[] args) {
        String[] a = new String[1];
        a[-1] = "Should throw ArrayIndexOutOfBoundsException";
    }
}