package arrays.objects.string_encoding_test;

public class StringEncodingTestOkMain {
    public static void main(String[] args) {
        test_ascii_strings();
        test_char_extraction();
        test_string_length();
        System.out.println("All string encoding tests passed");
    }

    static void test_ascii_strings() {
        String s1 = "a";
        String s2 = "ab";
        String s3 = "abc";

        assert s1.length() == 1;
        assert s2.length() == 2;
        assert s3.length() == 3;

        assert s1.charAt(0) == 'a';
        assert s2.charAt(0) == 'a';
        assert s2.charAt(1) == 'b';
        assert s3.charAt(0) == 'a';
        assert s3.charAt(1) == 'b';
        assert s3.charAt(2) == 'c';
    }

    static void test_char_extraction() {
        String s = "hello";
        char[] buf = new char[5];

        for (int i = 0; i < 5; i++) {
            buf[i] = s.charAt(i);
        }

        assert buf[0] == 'h';
        assert buf[1] == 'e';
        assert buf[2] == 'l';
        assert buf[3] == 'l';
        assert buf[4] == 'o';
    }

    static void test_string_length() {
        assert "".length() == 0;
        assert "a".length() == 1;
        assert "hello".length() == 5;
        assert "0123456789".length() == 10;
    }
}