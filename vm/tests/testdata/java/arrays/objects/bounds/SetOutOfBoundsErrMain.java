package arrays.objects.bounds.set_out_of_bounds;

public class SetOutOfBoundsErrMain {
    public static void main(String[] args) {
        String[] a = new String[1];
        a[2] = "Should throw ArrayIndexOutOfBoundsException";
    }
}