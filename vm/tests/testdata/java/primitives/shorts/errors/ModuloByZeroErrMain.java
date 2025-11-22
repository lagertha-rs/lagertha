package primitives.shorts.errors.modulo_by_zero;

public class ModuloByZeroErrMain {
    public static void main(String[] args) {
        short s = 0;
        var a = (short) (1 % s);
    }
}
