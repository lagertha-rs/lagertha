package arrays.primitives.set_negative_array_index_should_throw_err;

public class SetNegativeArrayIndexShouldThrowErrMain {
    public static void main(String[] args) {
        int[] a = new int[1];
        a[-1] = 42;
    }
}