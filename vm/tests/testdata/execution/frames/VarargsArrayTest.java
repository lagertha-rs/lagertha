// @test feature = "execution.frames.method-arguments"
// @test description = "Verifies empty, populated, explicit, and fixed-prefix array arguments."
// @test category = "success"

package execution.frames;

public class VarargsArrayTest {
    public static void main(String[] args) {
        assert sum() == 0 : "varargs.empty";
        assert sum(1, 2, 3, 4) == 10 : "varargs.populated";
        int[] explicit = new int[] { 5, 6 };
        assert sum(explicit) == 11 : "varargs.explicit.array";
        assert prefixedSum(10, 1, 2, 3) == 16 : "varargs.fixed.prefix";
    }

    static int sum(int... values) {
        int total = 0;
        for (int value : values) {
            total += value;
        }
        return total;
    }

    static int prefixedSum(int prefix, int... values) {
        return prefix + sum(values);
    }
}
