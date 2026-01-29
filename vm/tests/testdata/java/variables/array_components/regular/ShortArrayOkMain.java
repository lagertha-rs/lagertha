package variables.array_components.regular.short_array;

public class ShortArrayOkMain {
    public static void main(String[] args) {
        short[] arr = new short[3];
        arr[0] = 0;
        arr[1] = 32767;
        arr[2] = -32768;
        
        assert arr[0] == 0 : "short.0";
        assert arr[1] == Short.MAX_VALUE : "short.max";
        assert arr[2] == Short.MIN_VALUE : "short.min";
        assert arr.length == 3 : "short.length";
        
        System.out.println("Short array test passed.");
    }
}
