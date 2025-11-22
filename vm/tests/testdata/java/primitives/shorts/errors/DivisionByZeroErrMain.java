package primitives.shorts.errors.division_by_zero;

public class DivisionByZeroErrMain {
    public static void main(String[] args) {
        short s = 0;
        var a = (short) (1 / s);
    }
}
