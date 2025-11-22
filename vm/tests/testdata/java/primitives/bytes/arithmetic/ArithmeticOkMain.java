package primitives.bytes.arithmetic.comprehensive;

public class ArithmeticOkMain {
    public static void main(String[] args) {
        final byte BMAX = Byte.MAX_VALUE;   // 127
        final byte BMIN = Byte.MIN_VALUE;   // -128

        // Basic arithmetic & wraparound (must cast to byte to see wrapping)
        assert ((byte) (BMAX + 1)) == BMIN : "add.wrap";
        assert ((byte) (BMIN - 1)) == BMAX : "sub.wrap";
        assert ((byte) (50 * 50)) == (byte) 2500 : "mul.wrap"; // 2500 wraps to -44 in byte
        assert ((byte) (-BMIN)) == BMIN : "neg.edge";

        // Division & remainder
        assert ((byte) (7 / 3)) == 2 : "div.trunc.pos";
        assert ((byte) (-7 / 3)) == -2 : "div.trunc.negA";
        assert ((byte) (7 / -3)) == -2 : "div.trunc.negB";
        assert ((byte) (7 % 3)) == 1 : "rem.sign.pos";
        assert ((byte) (-7 % 3)) == -1 : "rem.sign.negA";
        assert ((byte) (7 % -3)) == 1 : "rem.sign.negB";

        byte a = -42, b = 7;
        assert (a / b) * b + (a % b) == a : "divrem.identity";
        assert ((byte) (BMIN / -1)) == BMIN : "min.div.minus1";
        assert ((byte) (BMIN % -1)) == 0 : "min.rem.minus1";

        // Shifts
        assert ((byte)(1 << 32)) == 1 : "shl.mask.32";
        assert ((byte)(1 << 33)) == 2 : "shl.mask.33";
        assert ((byte) (-2 >> 1)) == -1 : "shr.arith.neg";
        assert ((byte) (-2 >>> 1)) == -1 : "shr.logic.neg";

        // Bitwise operators
        assert ((byte) (~0)) == -1 : "bit.not";
        byte x = (byte) 0xAA, y = (byte) 0x0F;
        assert ((byte) (x & y)) == 0x0A : "bit.and";
        assert ((byte) (x | y)) == (byte) 0xAF : "bit.or";
        assert ((byte) (x ^ x)) == 0 : "bit.xor.self";

        // Comparisons
        assert (-1 < 1) : "cmp.signed.lt";
        assert (BMIN <= BMAX) : "cmp.signed.ge";
        assert (BMAX == 127) : "cmp.eq";

        System.out.println("All byte assertions passed.");
    }
}