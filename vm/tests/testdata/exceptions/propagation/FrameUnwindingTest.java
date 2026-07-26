// @test feature = "exceptions.propagation"
// @test description = "Verifies multi-frame, constructor, and replacement-exception propagation skips abruptly completed code."
// @test category = "success"

package exceptions.propagation;

public class FrameUnwindingTest {
    static int trace;

    public static void main(String[] args) {
        RuntimeException original = new RuntimeException("original");
        try {
            levelOne(original);
            assert false : "propagation.frames.missing.exception";
        } catch (RuntimeException caught) {
            assert caught == original : "propagation.frames.identity";
            assert trace == 123 : "propagation.frames.skipped.code";
        }

        try {
            new ThrowingConstructor();
            assert false : "propagation.constructor.missing.exception";
        } catch (IllegalArgumentException caught) {
            assert "constructor".equals(caught.getMessage()) : "propagation.constructor";
        }

        RuntimeException replacement = new RuntimeException("replacement");
        try {
            try {
                throw original;
            } catch (RuntimeException caught) {
                assert caught == original : "propagation.handler.original";
                throw replacement;
            }
        } catch (RuntimeException caught) {
            assert caught == replacement : "propagation.handler.replacement";
        }
    }

    static void levelOne(RuntimeException exception) {
        trace = trace * 10 + 1;
        levelTwo(exception);
        trace = -1;
    }

    static void levelTwo(RuntimeException exception) {
        trace = trace * 10 + 2;
        levelThree(exception);
        trace = -2;
    }

    static void levelThree(RuntimeException exception) {
        trace = trace * 10 + 3;
        throw exception;
    }
}

class ThrowingConstructor {
    ThrowingConstructor() {
        throw new IllegalArgumentException("constructor");
    }
}
