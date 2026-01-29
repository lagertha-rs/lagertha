package variables.array_components.regular.inc_dec;

public class IncDecOkMain {
    public static void main(String[] args) {
        int[] arr = new int[1];
        
        // Pre-increment
        arr[0] = 5;
        assert ++arr[0] == 6 : "++pre.result";
        assert arr[0] == 6 : "++pre.after";
        
        // Post-increment
        arr[0] = 5;
        assert arr[0]++ == 5 : "post++.result";
        assert arr[0] == 6 : "post++.after";
        
        // Pre-decrement
        arr[0] = 5;
        assert --arr[0] == 4 : "--pre.result";
        assert arr[0] == 4 : "--pre.after";
        
        // Post-decrement
        arr[0] = 5;
        assert arr[0]-- == 5 : "post--.result";
        assert arr[0] == 4 : "post--.after";
        
        // Byte wrapping
        byte[] bytes = new byte[1];
        bytes[0] = 127;
        bytes[0]++;
        assert bytes[0] == -128 : "byte.wrap.up";
        
        bytes[0] = -128;
        bytes[0]--;
        assert bytes[0] == 127 : "byte.wrap.down";
        
        System.out.println("Inc/dec test passed.");
    }
}
