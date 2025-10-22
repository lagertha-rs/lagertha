package primitives.int_arithmetic_ok;

public class IntArithmeticOkMain {
    public static void main(String[] args) {
        final int IMAX = Integer.MAX_VALUE;
        final int IMIN = Integer.MIN_VALUE;

        // Basic arithmetic & wraparound
        assert (IMAX + 1) == IMIN : "add.wrap";
        assert (IMIN - 1) == IMAX : "sub.wrap";
        assert (65536 * 65536) == 0 : "mul.wrap";
        assert (-IMIN) == IMIN : "neg.edge";

        // Division & remainder (no zero cases here)
        assert (7 / 3) == 2 : "div.trunc.pos";
        assert (-7 / 3) == -2 : "div.trunc.negA";
        assert (7 / -3) == -2 : "div.trunc.negB";
        assert (7 % 3) == 1 : "rem.sign.pos";
        assert (-7 % 3) == -1 : "rem.sign.negA";
        assert (7 % -3) == 1 : "rem.sign.negB";
        assert (-7 % -3) == -1 : "rem.sign.negBoth";

        int a = -12345, b = 67;
        assert (a / b) * b + (a % b) == a : "divrem.identity";
        assert (IMIN / -1) == IMIN : "min.div.minus1";
        assert (IMIN % -1) == 0 : "min.rem.minus1";

        // Shifts
        assert (1 << 32) == 1 : "shl.mask.32";
        assert (1 << 33) == 2 : "shl.mask.33";
        assert (-2 >> 1) == -1 : "shr.arith.neg";
        assert (-2 >>> 1) == 0x7FFFFFFF : "shr.logic.neg";
        assert (1 << -1) == (1 << 31) : "shl.negative.count";
        assert (1 >> 32) == 1 : "shr.mask.same";

        // Bitwise operators
        assert (~0) == -1 : "bit.not";
        int x = 0xAA55AA55, y = 0x0F0F0F0F;
        assert (x & y) == 0x0A050A05 : "bit.and";
        assert (x | y) == 0xAF5FAF5F : "bit.or";
        assert (x ^ x) == 0 : "bit.xor.self";

        // Narrowing casts (i2b, i2s, i2c semantics)
        assert ((byte) 256) == 0 : "cast.i2b.256";
        assert ((short) 65535) == -1 : "cast.i2s.65535";
        assert ((char) -1) == 65535 : "cast.i2c.minus1";
        assert ((int) ((char) 65535) + 1) == 65536 : "cast.char.promote";

        // Promotions & compound ops (iinc-like)
        int p = 1;
        p += IMAX;
        assert p == IMIN : "iinc.wrap";

        // Comparisons
        assert (-1 < 1) : "cmp.signed.lt";
        assert (IMIN <= IMAX) : "cmp.signed.ge";
        assert (123456789 == 123456789) : "cmp.eq";
        assert (123456789 != -123456789) : "cmp.ne";

        System.out.println("All assertions passed.");
    }
}