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
        Summable asInterface = new ClassImplementingSummable();
        ClassImplementingSummable asInstance = new ClassImplementingSummable();
        var asInterfaceRes = asInterface.sum(1, 2);
        var asInstanceRes = asInstance.sum(3, 4);
    }
}

