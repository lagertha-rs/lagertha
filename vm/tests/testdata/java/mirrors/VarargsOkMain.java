package mirrors.varargs_ok;

public class VarargsOkMain {
    public static void main(String[] args) {
        Object[] arr = new Object[2];
        arr[0] = "hello";
        arr[1] = "world";

        Class<?> arrClass = arr.getClass();
        Class<?> expected = Object[].class;

        assert arrClass == expected : "Array class mismatch";

        testVarargs("a", "b", "c");
    }

    static void testVarargs(Object... input) {
        assert input.getClass() == Object[].class : "Varargs class mismatch";
    }
}