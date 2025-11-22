package primitives.bytes.errors.division_by_zero;

public class DivisionByZeroErrMain {
    public static void main(String[] args) {
        byte b = 0;
        var a = (byte) (1 / b);
    }
}
