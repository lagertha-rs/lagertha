// @test feature = "exceptions.propagation"
// @test description = "Verifies finally paths execute after normal and abrupt completion and rethrow the original exception."
// @test category = "success"

package exceptions.propagation;

public class FinallyUnwindingTest {
    static int trace;

    public static void main(String[] args) {
        int result = normalCompletion();
        assert result == 7 : "finally.normal.result";
        assert trace == 12 : "finally.normal.order";

        RuntimeException original = new RuntimeException("abrupt");
        trace = 0;
        try {
            abruptCompletion(original);
            assert false : "finally.abrupt.missing.exception";
        } catch (RuntimeException caught) {
            assert caught == original : "finally.abrupt.identity";
            assert trace == 34 : "finally.abrupt.order";
        }
    }

    static int normalCompletion() {
        try {
            trace = 1;
            return 7;
        } finally {
            trace = trace * 10 + 2;
        }
    }

    static void abruptCompletion(RuntimeException exception) {
        try {
            trace = 3;
            throw exception;
        } finally {
            trace = trace * 10 + 4;
        }
    }
}
