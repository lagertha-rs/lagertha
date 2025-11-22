package primitives.shorts.arithmetic.comprehensive;

public class ArithmeticOkMain {
    public static void main(String[] args) {
        final short SMAX = Short.MAX_VALUE;   // 32767
        final short SMIN = Short.MIN_VALUE;   // -32768

        // Basic arithmetic & wraparound (must cast to short to see wrapping)
        assert ((short) (SMAX + 1)) == SMIN : "add.wrap";
        assert ((short) (SMIN - 1)) == SMAX : "sub.wrap";
        assert ((short) (256 * 256)) == 0 : "mul.wrap"; // 65536 wraps to 0
        assert ((short) (-SMIN)) == SMIN : "neg.edge";

        // Division & remainder
        assert ((short) (7 / 3)) == 2 : "div.trunc.pos";
        assert ((short) (-7 / 3)) == -2 : "div.trunc.negA";
        assert ((short) (7 / -3)) == -2 : "div.trunc.negB";
        assert ((short) (7 % 3)) == 1 : "rem.sign.pos";
        assert ((short) (-7 % 3)) == -1 : "rem.sign.negA";
        assert ((short) (7 % -3)) == 1 : "rem.sign.negB";

        short a = -12345, b = 67;
        assert (a / b) * b + (a % b) == a : "divrem.identity";
        assert ((short) (SMIN / -1)) == SMIN : "min.div.minus1";
        assert ((short) (SMIN % -1)) == 0 : "min.rem.minus1";

        // Shifts (shift distance is masked to 5 bits for int)
        assert ((short) (1 << 32)) == 1 : "shl.mask.32";
        assert ((short) (1 << 33)) == 2 : "shl.mask.33";
        assert ((short) (-2 >> 1)) == -1 : "shr.arith.neg";
        assert ((short) (-2 >>> 1)) == -1 : "shr.logic.neg";

        // Bitwise operators
        assert ((short) (~0)) == -1 : "bit.not";
        short x = (short) 0xAA55, y = (short) 0x0F0F;
        assert ((short) (x & y)) == 0x0A05 : "bit.and";
        assert ((short) (x | y)) == (short) 0xAF5F : "bit.or";
        assert ((short) (x ^ x)) == 0 : "bit.xor.self";

        // Comparisons
        assert (-1 < 1) : "cmp.signed.lt";
        assert (SMIN <= SMAX) : "cmp.signed.ge";
        assert (SMAX == 32767) : "cmp.eq";

        System.out.println("All short assertions passed.");
    }
}