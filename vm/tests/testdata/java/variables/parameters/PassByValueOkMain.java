package variables.parameters.pass_by_value;

public class PassByValueOkMain {
    public static void main(String[] args) {
        // Modifying parameter doesn't affect caller's variable
        int x = 10;
        modifyInt(x);
        assert x == 10 : "pbv.int.unchanged";

        long y = 100L;
        modifyLong(y);
        assert y == 100L : "pbv.long.unchanged";

        boolean b = true;
        modifyBool(b);
        assert b == true : "pbv.bool.unchanged";

        double d = 3.14;
        modifyDouble(d);
        assert d == 3.14 : "pbv.double.unchanged";

        System.out.println("Pass by value test passed.");
    }

    static void modifyInt(int x) {
        x = 999; // Local modification, doesn't affect caller
    }

    static void modifyLong(long x) {
        x = 999L;
    }

    static void modifyBool(boolean x) {
        x = false;
    }

    static void modifyDouble(double x) {
        x = 999.0;
    }
}
