package variables.parameters.recursive_parameters;

public class RecursiveParametersOkMain {
    public static void main(String[] args) {
        // Factorial
        assert factorial(0) == 1 : "rec.fact.0";
        assert factorial(1) == 1 : "rec.fact.1";
        assert factorial(5) == 120 : "rec.fact.5";

        // Fibonacci
        assert fibonacci(0) == 0 : "rec.fib.0";
        assert fibonacci(1) == 1 : "rec.fib.1";
        assert fibonacci(10) == 55 : "rec.fib.10";

        // Sum to N
        assert sumToN(0) == 0 : "rec.sum.0";
        assert sumToN(5) == 15 : "rec.sum.5";
        assert sumToN(10) == 55 : "rec.sum.10";

        System.out.println("Recursive parameters test passed.");
    }

    static int factorial(int n) {
        if (n <= 1) return 1;
        return n * factorial(n - 1);
    }

    static int fibonacci(int n) {
        if (n <= 0) return 0;
        if (n == 1) return 1;
        return fibonacci(n - 1) + fibonacci(n - 2);
    }

    static int sumToN(int n) {
        if (n <= 0) return 0;
        return n + sumToN(n - 1);
    }
}
