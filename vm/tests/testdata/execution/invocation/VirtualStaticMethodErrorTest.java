// @test feature = "execution.invocation.virtual"
// @test description = "Throws IncompatibleClassChangeError when invokevirtual resolves a static class method."
// @test category = "error"

package execution.invocation;

public class VirtualStaticMethodErrorTest {
    public static void main(String[] args) {
        new VirtualStaticTarget().value();
    }
}

class VirtualStaticTarget {
    int value() {
        return 1;
    }
}
