package variables.parameters.no_parameters;

public class NoParametersOkMain {
    public static void main(String[] args) {
        assert noParamMethod() == 42 : "noparam.return";
        System.out.println("No parameters test passed.");
    }

    static int noParamMethod() {
        return 42;
    }
}
