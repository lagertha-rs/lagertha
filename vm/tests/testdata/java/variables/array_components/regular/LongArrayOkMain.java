package variables.array_components.regular.long_array;

public class LongArrayOkMain {
    public static void main(String[] args) {
        long[] arr = new long[3];
        arr[0] = 0L;
        arr[1] = Long.MAX_VALUE;
        arr[2] = Long.MIN_VALUE;
        
        assert arr[0] == 0L : "long.0";
        assert arr[1] == 9223372036854775807L : "long.max";
        assert arr[2] == -9223372036854775808L : "long.min";
        assert arr.length == 3 : "long.length";
        
        System.out.println("Long array test passed.");
    }
}
