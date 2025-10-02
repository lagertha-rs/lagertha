package arrays.system_array_copy_primitives;

public class SystemArrayCopyPrimitivesMain {
    public static void main(String[] args) {
        int[] src = {1, 2, 3, 4, 5};
        int[] dest = new int[5];

        System.arraycopy(src, 1, dest, 0, 3);
    }
}