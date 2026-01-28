package variables.instance.field_shadowing;

public class FieldShadowingOkMain {
    public static void main(String[] args) {
        ShadowingHolder holder = new ShadowingHolder();

        // Local shadows field
        assert holder.testLocalShadow() == true : "shadow.local";

        // Parameter shadows field
        assert holder.testParameterShadow(777) == true : "shadow.param";

        System.out.println("Field shadowing test passed.");
    }
}

class ShadowingHolder {
    int value = 100;

    boolean testLocalShadow() {
        int value = 999; // shadows field
        // Local is 999, field is 100
        return value == 999 && this.value == 100;
    }

    boolean testParameterShadow(int value) {
        // Parameter is 777, field is 100
        return value == 777 && this.value == 100;
    }
}
