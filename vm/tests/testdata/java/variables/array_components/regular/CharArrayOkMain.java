package variables.array_components.regular.char_array;

public class CharArrayOkMain {
    public static void main(String[] args) {
        char[] arr = new char[5];
        arr[0] = 'A';
        arr[1] = '\u0000';
        arr[2] = '\uFFFF';
        arr[3] = (char)255;
        arr[4] = (char)0x1234;
        
        assert arr[0] == 'A' : "char.A";
        assert arr[1] == 0 : "char.min";
        assert arr[2] == 65535 : "char.max";
        assert arr[3] == 255 : "char.255";
        assert arr[4] == 0x1234 : "char.unicode";
        assert arr.length == 5 : "char.length";
        
        // Test System.arraycopy
        char[] src = {'x', 'y', 'z'};
        char[] dst = new char[3];
        System.arraycopy(src, 0, dst, 0, 3);
        assert dst[0] == 'x' : "arraycopy.0";
        assert dst[1] == 'y' : "arraycopy.1";
        assert dst[2] == 'z' : "arraycopy.2";
        
        // Test String.charAt interaction
        String s = "test";
        assert s.charAt(0) == 't' : "charAt.0";
        assert s.charAt(3) == 't' : "charAt.3";
        
        System.out.println("Char array test passed.");
    }
}
