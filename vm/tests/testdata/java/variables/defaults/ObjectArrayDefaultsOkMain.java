package variables.defaults.object_array_defaults;

public class ObjectArrayDefaultsOkMain {
    public static void main(String[] args) {
        // Object array elements default to null
        Object[] objArr = new Object[5];
        for (int i = 0; i < 5; i++) {
            assert objArr[i] == null : "arr.obj.default";
        }

        // String array elements default to null
        String[] strArr = new String[5];
        for (int i = 0; i < 5; i++) {
            assert strArr[i] == null : "arr.str.default";
        }

        // Custom class array elements default to null
        Holder[] holderArr = new Holder[5];
        for (int i = 0; i < 5; i++) {
            assert holderArr[i] == null : "arr.holder.default";
        }

        // Array of arrays elements default to null
        int[][] arrOfArr = new int[5][];
        for (int i = 0; i < 5; i++) {
            assert arrOfArr[i] == null : "arr.arr.default";
        }

        System.out.println("Object array defaults test passed.");
    }
}

class Holder {
    int value;
}
