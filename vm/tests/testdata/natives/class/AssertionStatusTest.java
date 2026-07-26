// @test feature = "natives.class.assertion-status"
// @test description = "Verifies assertions are enabled, true conditions complete, and false details construct AssertionError."
// @test category = "success"

package natives.classstatus;

public class AssertionStatusTest {
    public static void main(String[] args) {
        boolean enabled = false;
        assert enabled = true;
        if (!enabled) {
            throw new RuntimeException("assertions disabled");
        }

        assert true : "true assertion must complete";
        try {
            assert false : "assertion-detail";
            throw new RuntimeException("false assertion completed");
        } catch (AssertionError expected) {
            if (!"assertion-detail".equals(expected.getMessage())) {
                throw new RuntimeException("wrong assertion detail");
            }
        }
    }
}
