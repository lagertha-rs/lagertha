package arrays.primitives.basic.byte_array;

public class ByteArrayOkMain {
    public static void main(String[] args) {
        test_basic_byte_array();
        test_byte_values();
        test_byte_array_copy();
        System.out.println("All byte array tests passed");
    }

    static void test_basic_byte_array() {
        byte[] arr = new byte[5];
        arr[0] = 1;
        arr[1] = 2;
        arr[2] = -1;
        arr[3] = 127;
        arr[4] = -128;

        assert arr[0] == 1;
        assert arr[1] == 2;
        assert arr[2] == -1;
        assert arr[3] == 127;
        assert arr[4] == -128;
        assert arr.length == 5;
    }

    static void test_byte_values() {
        byte[] arr = new byte[3];
        arr[0] = (byte)0;
        arr[1] = (byte)255;  // will be -1
        arr[2] = (byte)65;   // 'A'

        assert arr[0] == 0;
        assert arr[1] == -1;  // 255 as signed byte
        assert arr[2] == 65;
    }

    static void test_byte_array_copy() {
        byte[] src = {10, 20, 30};
        byte[] dst = new byte[3];
        System.arraycopy(src, 0, dst, 0, 3);

        assert dst[0] == 10;
        assert dst[1] == 20;
        assert dst[2] == 30;
    }
}