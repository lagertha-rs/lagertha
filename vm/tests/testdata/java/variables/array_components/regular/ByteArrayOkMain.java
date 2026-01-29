package variables.array_components.regular.byte_array;

public class ByteArrayOkMain {
    public static void main(String[] args) {
        byte[] arr = new byte[5];
        arr[0] = 0;
        arr[1] = 127;
        arr[2] = -128;
        arr[3] = (byte)255;  // wraps to -1
        arr[4] = (byte)65;   // 'A'
        
        assert arr[0] == 0 : "byte.0";
        assert arr[1] == 127 : "byte.max";
        assert arr[2] == -128 : "byte.min";
        assert arr[3] == -1 : "byte.255";
        assert arr[4] == 65 : "byte.65";
        assert arr.length == 5 : "byte.length";
        
        // Test System.arraycopy
        byte[] src = {10, 20, 30};
        byte[] dst = new byte[3];
        System.arraycopy(src, 0, dst, 0, 3);
        assert dst[0] == 10 : "arraycopy.0";
        assert dst[1] == 20 : "arraycopy.1";
        assert dst[2] == 30 : "arraycopy.2";
        
        System.out.println("Byte array test passed.");
    }
}
