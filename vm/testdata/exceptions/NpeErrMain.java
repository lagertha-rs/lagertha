package exceptions.npe;

public class NpeErrMain {
    static void method2() {
        throw new NullPointerException("I want to see the stacktrace and error messages");
    }

    static void method1() {
        method2();
    }

    public static void main(String[] args) {
        method1();
    }
}