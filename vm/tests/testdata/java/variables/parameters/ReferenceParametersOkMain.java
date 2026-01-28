package variables.parameters.reference_parameters;

public class ReferenceParametersOkMain {
    public static void main(String[] args) {
        // Object parameter
        Object obj = new Object();
        assert objectParam(obj) == obj : "ref.object";

        // String parameter
        String str = "test";
        assert stringParam(str) == "test" : "ref.string";

        // Custom class parameter
        Holder holder = new Holder(42);
        assert holderParam(holder).value == 42 : "ref.holder";

        System.out.println("Reference parameters test passed.");
    }

    static Object objectParam(Object o) { return o; }
    static String stringParam(String s) { return s; }
    static Holder holderParam(Holder h) { return h; }
}

class Holder {
    int value;
    Holder(int v) { this.value = v; }
}
