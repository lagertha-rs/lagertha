package variables.parameters.parameter_shadowing;

public class ParameterShadowingOkMain {
    // Instance field for shadowing tests
    int value = 100;
    
    public static void main(String[] args) {
        ParameterShadowingOkMain instance = new ParameterShadowingOkMain();
        
        // Parameter shadows instance field
        assert instance.shadowTest(50) == true : "shadow.param";

        // Instance field unchanged
        assert instance.value == 100 : "shadow.field.unchanged";

        System.out.println("Parameter shadowing test passed.");
    }

    boolean shadowTest(int value) {
        // Parameter 'value' shadows field 'value'
        // Parameter is 50, field is 100
        return value == 50 && this.value == 100;
    }
}
