package primitives.longs.arithmetic.comprehensive;

public class ArithmeticOkMain {
    public static void main(String[] args) {
        final long LMAX = Long.MAX_VALUE;
        final long LMIN = Long.MIN_VALUE;

        // Basic arithmetic & wraparound
        assert (LMAX + 1) == LMIN : "add.wrap";
        assert (LMIN - 1) == LMAX : "sub.wrap";
        assert (0x100000000L * 0x100000000L) == 0L : "mul.wrap";
        assert (-LMIN) == LMIN : "neg.edge";

        // Division & remainder
        assert (7L / 3L) == 2L : "div.trunc.pos";
        assert (-7L / 3L) == -2L : "div.trunc.negA";
        assert (7L / -3L) == -2L : "div.trunc.negB";
        assert (7L % 3L) == 1L : "rem.sign.pos";
        assert (-7L % 3L) == -1L : "rem.sign.negA";
        assert (7L % -3L) == 1L : "rem.sign.negB";
        assert (-7L % -3L) == -1L : "rem.sign.negBoth";

        long a = -123456789L, b = 67L;
        assert (a / b) * b + (a % b) == a : "divrem.identity";
        assert (LMIN / -1L) == LMIN : "min.div.minus1";
        assert (LMIN % -1L) == 0L : "min.rem.minus1";

        // Shifts
        assert (1L << 64) == 1L : "shl.mask.64";
        assert (1L << 65) == 2L : "shl.mask.65";
        assert (-2L >> 1) == -1L : "shr.arith.neg";
        assert (-2L >>> 1) == 0x7FFFFFFFFFFFFFFFL : "shr.logic.neg";
        assert (1L << -1) == (1L << 63) : "shl.negative.count";
        assert (1L >> 64) == 1L : "shr.mask.same";

        // Bitwise operators
        assert (~0L) == -1L : "bit.not";
        long x = 0xAA55AA55AA55AA55L, y = 0x0F0F0F0F0F0F0F0FL;
        assert ((x & y)) == 0x0A050A050A050A05L : "bit.and";
        assert ((x | y)) == 0xAF5FAF5FAF5FAF5FL : "bit.or";
        assert ((x ^ x)) == 0L : "bit.xor.self";

        // Comparisons
        assert (-1L < 1L) : "cmp.signed.lt";
        assert (LMIN <= LMAX) : "cmp.signed.ge";
        assert (123456789L == 123456789L) : "cmp.eq";
        assert (123456789L != -123456789L) : "cmp.ne";

        // Compound ops
        long p = 1L;
        p += LMAX;
        assert p == LMIN : "compound.wrap";

        System.out.println("All long assertions passed.");
    }
}