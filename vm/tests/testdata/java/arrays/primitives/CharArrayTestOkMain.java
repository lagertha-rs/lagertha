package arrays.primitives.char_array_test;

public class CharArrayTestOkMain {
    public static void main(String[] args) {
        test_basic_char_array();
        test_char_values();
        test_char_array_clone();
        test_string_value_field();
        System.out.println("All char array tests passed");
    }

    static void test_basic_char_array() {
        char[] arr = new char[5];
        arr[0] = 'a';
        arr[1] = 'b';
        arr[2] = 'c';
        arr[3] = 'd';
        arr[4] = 'e';

        assert arr[0] == 'a' : "index 0";
        assert arr[1] == 'b' : "index 1";
        assert arr[2] == 'c' : "index 2";
        assert arr[3] == 'd' : "index 3";
        assert arr[4] == 'e' : "index 4";
        assert arr.length == 5 : "length";
    }

    static void test_char_values() {
        char[] arr = new char[4];
        arr[0] = (char)0;
        arr[1] = (char)65;    // 'A'
        arr[2] = (char)255;   // ÿ
        arr[3] = (char)0x1234; // some unicode

        assert arr[0] == 0;
        assert arr[1] == 65;
        assert arr[2] == 255;
        assert arr[3] == 0x1234;
    }

    static void test_char_array_clone() {
        char[] src = {'x', 'y', 'z'};
        char[] dst = new char[3];
        System.arraycopy(src, 0, dst, 0, 3);

        assert dst[0] == 'x';
        assert dst[1] == 'y';
        assert dst[2] == 'z';
    }

    static void test_string_value_field() {
        // Test that String properly wraps a char array
        String s = "test";
        assert s.length() == 4;
        assert s.charAt(0) == 't';
        assert s.charAt(1) == 'e';
        assert s.charAt(2) == 's';
        assert s.charAt(3) == 't';
    }
}