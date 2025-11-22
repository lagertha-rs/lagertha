package primitives.bytes.errors.modulo_by_zero;

public class ModuloByZeroErrMain {
    public static void main(String[] args) {
        byte b = 0;
        var a = (byte) (1 % b);
    }
}
