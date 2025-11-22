package natives.system.arraycopy.basic;

public class ArrayCopyOkMain {
    public static void main(String[] args) {
        test_int_copy();
        test_partial_copy();
        test_overlap_forward();
        test_overlap_backward();
        test_object_copy();
        test_zero_length_copy();
    }

    static void test_int_copy() {
        int[] src = {1, 2, 3, 4, 5};
        int[] dst = new int[5];
        System.arraycopy(src, 0, dst, 0, 5);
        assert dst[0] == 1;
        assert dst[4] == 5;
    }

    static void test_partial_copy() {
        int[] src = {10, 20, 30, 40, 50};
        int[] dst = new int[5];
        System.arraycopy(src, 1, dst, 0, 3);
        assert dst[0] == 20;
        assert dst[2] == 40;
        assert dst[3] == 0; // untouched
    }

    static void test_overlap_forward() {
        int[] arr = {0, 1, 2, 3, 4, 5};
        System.arraycopy(arr, 0, arr, 2, 4); // overlapping, safe forward copy
        assert arr[2] == 0;
        assert arr[3] == 1;
        assert arr[4] == 2;
        assert arr[5] == 3;
    }

    static void test_overlap_backward() {
        int[] arr = {0, 1, 2, 3, 4, 5};
        System.arraycopy(arr, 2, arr, 0, 4); // overlapping, safe backward copy
        assert arr[0] == 2;
        assert arr[1] == 3;
        assert arr[2] == 4;
        assert arr[3] == 5;
    }

    static void test_object_copy() {
        String[] src = {"a", "b", "c", null};
        String[] dst = new String[4];
        System.arraycopy(src, 0, dst, 0, src.length);
        assert dst[0].equals("a");
        assert dst[2].equals("c");
        assert dst[3] == null;
    }

    static void test_zero_length_copy() {
        int[] src = {1, 2, 3};
        int[] dst = {4, 5, 6};
        System.arraycopy(src, 0, dst, 0, 0);
        assert dst[0] == 4; // nothing changed
        assert dst[2] == 6;
    }
}
