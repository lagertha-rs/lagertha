package variables.parameters.null_parameters;

public class NullParametersOkMain {
    public static void main(String[] args) {
        // Pass null Object
        assert isNull(null) == true : "null.object";
        assert isNull(new Object()) == false : "null.object.not";

        // Pass null String
        assert isNullString(null) == true : "null.string";
        assert isNullString("hello") == false : "null.string.not";

        // Pass null array
        assert isNullArray(null) == true : "null.array";
        assert isNullArray(new int[5]) == false : "null.array.not";

        System.out.println("Null parameters test passed.");
    }

    static boolean isNull(Object o) {
        return o == null;
    }

    static boolean isNullString(String s) {
        return s == null;
    }

    static boolean isNullArray(int[] arr) {
        return arr == null;
    }
}
