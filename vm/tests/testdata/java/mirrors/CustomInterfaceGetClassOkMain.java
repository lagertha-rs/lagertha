package mirrors.custom_interface_getclass;

public class CustomInterfaceGetClassOkMain {
    interface MyInterface {}

    static class MyImpl implements MyInterface {}

    public static void main(String[] args) {
        MyInterface obj = new MyImpl();

        Class<?> c = obj.getClass();

        assert c == MyImpl.class : "Should be MyImpl.class";
        assert c != MyInterface.class : "Should not be interface class";

        System.out.println(c.getName());
    }
}