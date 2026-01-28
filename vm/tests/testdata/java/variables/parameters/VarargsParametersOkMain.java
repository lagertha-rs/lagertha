package variables.parameters.varargs_parameters;

public class VarargsParametersOkMain {
    public static void main(String[] args) {
        // No args
        assert sumVarargs() == 0 : "varargs.none";

        // Single arg
        assert sumVarargs(10) == 10 : "varargs.one";

        // Multiple args
        assert sumVarargs(1, 2, 3) == 6 : "varargs.three";
        assert sumVarargs(1, 2, 3, 4, 5) == 15 : "varargs.five";

        // Mixed regular and varargs
        assert mixedVarargs(100, 1, 2, 3) == 106 : "varargs.mixed";

        // Pass array as varargs
        int[] arr = {10, 20, 30};
        assert sumVarargs(arr) == 60 : "varargs.array";

        System.out.println("Varargs parameters test passed.");
    }

    static int sumVarargs(int... nums) {
        int sum = 0;
        for (int i = 0; i < nums.length; i++) {
            sum += nums[i];
        }
        return sum;
    }

    static int mixedVarargs(int base, int... nums) {
        int sum = base;
        for (int i = 0; i < nums.length; i++) {
            sum += nums[i];
        }
        return sum;
    }
}
