package exception.runtime_exception_get_stack_trace;

public class ThrowRuntimeExceptionGetStackTrace {
    static void secondCall() {
        throw new NullPointerException();
    }

    static void firstCall() {
        secondCall();
    }

    public static void main(String[] args) {
        try {
            firstCall();
        }  catch (RuntimeException e) {
            var a = e.getStackTrace();
        }
    }
}