package primitives.int_arithmetic_ok;

public class IntArithmeticOkMain {
    private static int passed = 0, total = 0;

    private static void t(String name, boolean ok) {
        total++;
        System.out.println(name.concat(":").concat(ok ? "OK" : "FAIL"));
        if (ok) passed++;
    }

    public static void main(String[] args) {
        final int IMAX = Integer.MAX_VALUE;
        final int IMIN = Integer.MIN_VALUE;

        // A) Basic arithmetic & wraparound
        t("add.wrap", IMAX + 1 == IMIN);
        t("sub.wrap", IMIN - 1 == IMAX);
        t("mul.wrap", (65536 * 65536) == 0);                // 2^16 * 2^16 = 2^32 -> wrap to 0
        t("neg.edge", -IMIN == IMIN);                       // two's-complement edge

        // B) Division & remainder (no zero cases here)
        t("div.trunc.pos", 7 / 3 == 2);
        t("div.trunc.negA", -7 / 3 == -2);
        t("div.trunc.negB", 7 / -3 == -2);
        t("rem.sign.pos", 7 % 3 == 1);
        t("rem.sign.negA", -7 % 3 == -1);
        t("rem.sign.negB", 7 % -3 == 1);
        t("rem.sign.negBoth", -7 % -3 == -1);
        int a = -12345, b = 67;
        t("divrem.identity", (a / b) * b + (a % b) == a);
        t("min.div.minus1", (IMIN / -1) == IMIN);           // wraps, no throw
        t("min.rem.minus1", (IMIN % -1) == 0);

        // C) Shifts (masking of shift distance; >> vs >>>)
        t("shl.mask.32", (1 << 32) == 1);
        t("shl.mask.33", (1 << 33) == 2);
        t("shr.arith.neg", (-2 >> 1) == -1);                // sign-extend
        t("shr.logic.neg", (-2 >>> 1) == 0x7FFFFFFF);       // zero-extend
        t("shl.negative.count", (1 << -1) == (1 << 31));    // -1 & 31 = 31
        t("shr.mask.same", (1 >> 32) == 1);

        // D) Bitwise operators
        t("bit.not", ~0 == -1);
        int x = 0xAA55AA55, y = 0x0F0F0F0F;
        t("bit.and", (x & y) == 0x0A050A05);
        t("bit.or", (x | y) == 0xAF5FAF5F);
        t("bit.xor.self", (x ^ x) == 0);

        // E) Narrowing casts (i2b, i2s, i2c semantics)
        t("cast.i2b.256", ((byte) 256) == 0);
        t("cast.i2s.65535", ((short) 65535) == -1);
        t("cast.i2c.minus1", ((char) -1) == 65535);
        t("cast.char.promote", ((int) ((char) 65535) + 1) == 65536);

        // F) Promotions & compound ops (iinc-like)
        int p = 1;
        p += IMAX; // same as p = p + IMAX (wrap)
        t("iinc.wrap", p == IMIN);

        // G) Comparisons
        t("cmp.signed.lt", (-1 < 1));
        t("cmp.signed.ge", (IMIN <= IMAX));
        t("cmp.eq", (123456789 == 123456789));
        t("cmp.ne", (123456789 != -123456789));

        System.out.println("TOTAL:".concat(String.valueOf(passed)).concat("/").concat(String.valueOf(total)));
    }
}