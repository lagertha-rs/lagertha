package primitives.chars.errors.modulo_by_zero;

public class ModuloByZeroErrMain {
    public static void main(String[] args) {
        char c = 0;
        var a = (char) (1 % c);
    }
}
