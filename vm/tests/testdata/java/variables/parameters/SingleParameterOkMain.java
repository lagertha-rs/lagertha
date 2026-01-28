package variables.parameters.single_parameter;

public class SingleParameterOkMain {
    public static void main(String[] args) {
        assert singleInt(10) == 20 : "single.int";
        assert singleLong(100L) == 200L : "single.long";
        assert singleBool(true) == false : "single.bool";
        assert singleString("hello") == "hello" : "single.string";
        System.out.println("Single parameter test passed.");
    }

    static int singleInt(int x) {
        return x * 2;
    }

    static long singleLong(long x) {
        return x * 2;
    }

    static boolean singleBool(boolean x) {
        return !x;
    }

    static String singleString(String s) {
        return s;
    }
}
