package variables.parameters.multiple_parameters;

public class MultipleParametersOkMain {
    public static void main(String[] args) {
        // Two parameters
        assert twoParams(10, 20) == 30 : "two.add";

        // Three parameters
        assert threeParams(1, 2, 3) == 6 : "three.add";

        // Many parameters
        assert manyParams(1, 2, 3, 4, 5, 6, 7, 8) == 36 : "many.add";

        // Mixed types
        assert mixedParams(10, 3.5f, true) == 13 : "mixed.types";

        // Order matters
        assert orderedParams(100, 10) == 10 : "order.div";
        assert orderedParams(10, 100) == 0 : "order.div.reversed";

        System.out.println("Multiple parameters test passed.");
    }

    static int twoParams(int a, int b) {
        return a + b;
    }

    static int threeParams(int a, int b, int c) {
        return a + b + c;
    }

    static int manyParams(int a, int b, int c, int d, int e, int f, int g, int h) {
        return a + b + c + d + e + f + g + h;
    }

    static int mixedParams(int i, float f, boolean b) {
        return i + (int) f + (b ? 0 : 1);
    }

    static int orderedParams(int dividend, int divisor) {
        return dividend / divisor;
    }
}
