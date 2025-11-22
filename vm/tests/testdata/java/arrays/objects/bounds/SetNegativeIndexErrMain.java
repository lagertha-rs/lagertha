package arrays.objects.bounds.set_negative_index;

public class SetNegativeIndexErrMain {
    public static void main(String[] args) {
        String[] a = new String[1];
        a[-1] = "Should throw ArrayIndexOutOfBoundsException";
    }
}