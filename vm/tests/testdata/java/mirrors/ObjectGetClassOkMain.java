package mirrors.object_getclass;

public class ObjectGetClassOkMain {
    static class MyClass {}

    public static void main(String[] args) {
        Object o = new MyClass();
        Class<?> c1 = o.getClass();

        assert c1 == MyClass.class : "Object reference getClass mismatch";

        MyClass m = new MyClass();
        Class<?> c2 = m.getClass();

        assert c2 == MyClass.class : "Direct getClass mismatch";
        assert c1 == c2 : "Same class should return same Class object";

        System.out.println(c1.getName());
    }
}