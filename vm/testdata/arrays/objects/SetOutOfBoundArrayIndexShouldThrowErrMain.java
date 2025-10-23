package arrays.objects.set_out_of_bound_array_index_should_throw_err;

public class SetOutOfBoundArrayIndexShouldThrowErrMain {
    public static void main(String[] args) {
        String[] a = new String[1];
        a[2] = "Should throw ArrayIndexOutOfBoundsException";
    }
}