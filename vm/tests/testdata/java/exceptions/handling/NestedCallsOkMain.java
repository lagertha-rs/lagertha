package exceptions.handling.nested_calls;

public class NestedCallsOkMain {
    static void level3() {
        throw new NullPointerException("From level3");
    }

    static void level2() {
        level3();
        System.out.println("This is not executed");
    }

    static void level1() {
        level2();
        System.out.println("This is not executed");
    }

    public static void main(String[] args) {
        try {
            level1();
            System.out.println("This line should not be printed");
        } catch (NullPointerException e) {
            System.out.println("Caught NPE from nested calls");
        }
        System.out.println("Execution continues after catch");
    }
}