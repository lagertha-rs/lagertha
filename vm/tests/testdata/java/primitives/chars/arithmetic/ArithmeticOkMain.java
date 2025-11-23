package primitives.chars.arithmetic.comprehensive;

public class ArithmeticOkMain {
    public static void main(String[] args) {
        final char CMAX = Character.MAX_VALUE;   // 65535 (0xFFFF)
        final char CMIN = Character.MIN_VALUE;   // 0

        // Basic arithmetic & wraparound (char is unsigned, promoted to int in operations)
        assert ((char) (CMAX + 1)) == CMIN : "add.wrap";
        assert ((char) (CMIN - 1)) == CMAX : "sub.wrap";
        assert ((char) (256 * 256)) == 0 : "mul.wrap"; // 65536 wraps to 0
        
        // Char is always non-negative (unsigned)
        assert ((int) CMIN) == 0 : "min.unsigned";
        assert ((int) CMAX) == 65535 : "max.unsigned";

        // Division & remainder (char promoted to int)
        char c1 = 7, c2 = 3;
        assert ((char) (c1 / c2)) == 2 : "div.pos";
        assert ((char) (c1 % c2)) == 1 : "rem.pos";

        char a = 100, b = 7;
        assert (a / b) * b + (a % b) == a : "divrem.identity";

        // Shifts (shift distance is masked to 5 bits for int promotion)
        assert ((char) (1 << 32)) == 1 : "shl.mask.16";
        assert ((char) (1 << 33)) == 2 : "shl.mask.17";
        char c = (char) 0xFFFE; // -2 as signed, but unsigned in char context
        assert ((char) (c >> 1)) == 0x7FFF : "shr.unsigned.promoted";
        assert ((char) (c >>> 1)) == 0x7FFF : "shr.logic.same";

        // Bitwise operators
        assert ((char) (~0)) == (char) 0xFFFF : "bit.not";
        char x = (char) 0xAA55, y = (char) 0x0F0F;
        assert ((char) (x & y)) == 0x0A05 : "bit.and";
        assert ((char) (x | y)) == (char) 0xAF5F : "bit.or";
        assert ((char) (x ^ x)) == 0 : "bit.xor.self";

        // Char-specific: Unicode and character literals
        assert 'A' == 65 : "char.literal.ascii";
        assert '\u0041' == 'A' : "char.unicode.escape";
        assert '\n' == 10 : "char.newline";
        assert '\t' == 9 : "char.tab";
        assert '0' == 48 : "char.digit.zero";
        assert '9' == 57 : "char.digit.nine";

        // Casting to/from char
        assert ((char) -1) == 65535 : "cast.i2c.minus1";
        assert ((char) 65536) == 0 : "cast.i2c.wrap";
        assert ((int) ((char) -1)) == 65535 : "cast.c2i.unsigned";
        
        // Char to byte (truncates to lower 8 bits)
        assert ((byte) 'A') == 65 : "cast.c2b.ascii";
        assert ((byte) '\u00FF') == -1 : "cast.c2b.high";
        assert ((byte) '\u0100') == 0 : "cast.c2b.wrap";
        
        // Char to short (may truncate, sign extends)
        assert ((short) 'A') == 65 : "cast.c2s.ascii";
        assert ((short) '\uFFFF') == -1 : "cast.c2s.max";
        
        // Comparisons (unsigned semantics)
        assert ('\u0000' < '\u0001') : "cmp.unsigned.lt";
        assert ('\uFFFF' > '\u0000') : "cmp.unsigned.gt";
        assert ('A' == 'A') : "cmp.eq";
        assert ('A' != 'B') : "cmp.ne";
        
        // Char in expressions promotes to int
        char ch = 'A';
        int promoted = ch + 1; // Promoted to int
        assert promoted == 66 : "promotion.to.int";
        
        // Compound operations
        char p = 1;
        p += CMAX;
        assert p == CMIN : "compound.wrap";

        System.out.println("All char assertions passed.");
    }
}
