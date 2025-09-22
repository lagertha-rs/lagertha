package interfaces.basic_interface;

public class BasicInterfaceMain {
    interface Summable {
        int sum(int a, int b);
    }

    static class ClassImplementingSummable implements Summable {
        @Override
        public int sum(int a, int b) {
            return a + b;
        }
    }

    public static void main(String[] args) {
        var instance = new ClassImplementingSummable();
        var res = instance.sum(1, 2);
    }
}

