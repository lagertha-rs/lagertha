package exception.runtime_exception;

public class ThrowRuntimeException {
    static void secondCall() {
        throw new NullPointerException();
    }

    static void firstCall() {
        secondCall();
    }

    public static void main(String[] args) {
        firstCall();
    }
}