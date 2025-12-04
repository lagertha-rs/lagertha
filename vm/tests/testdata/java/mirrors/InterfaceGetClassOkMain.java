package mirrors.interface_getclass;

import java.util.Map;

public class InterfaceGetClassOkMain {
    public static void main(String[] args) {
        Map<String, String> m = Map.of();
        Class<?> c = m.getClass();

        assert c != null : "getClass returned null";
        assert c != Map.class : "Should be implementation class, not Map interface";

        System.out.println(c.getName());
    }
}