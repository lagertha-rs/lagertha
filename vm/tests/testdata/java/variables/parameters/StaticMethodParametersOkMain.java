package variables.parameters.static_method_parameters;

public class StaticMethodParametersOkMain {
    public static void main(String[] args) {
        // Static method with primitive
        assert StaticMethods.staticAdd(10, 20) == 30 : "static.add";

        // Static method with object
        Holder h = new Holder(5);
        assert StaticMethods.staticReadHolder(h) == 5 : "static.read";

        // Static method with array
        int[] arr = {1, 2, 3};
        assert StaticMethods.staticSumArray(arr) == 6 : "static.sum";

        System.out.println("Static method parameters test passed.");
    }
}

class Holder {
    int value;
    Holder(int v) { this.value = v; }
}

class StaticMethods {
    static int staticAdd(int a, int b) {
        return a + b;
    }

    static int staticReadHolder(Holder h) {
        return h.value;
    }

    static int staticSumArray(int[] arr) {
        int sum = 0;
        for (int i = 0; i < arr.length; i++) {
            sum += arr[i];
        }
        return sum;
    }
}
