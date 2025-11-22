package primitives.conversions.comprehensive;

public class ConversionsOkMain {
    public static void main(String[] args) {
        // Widening conversions (implicit, no data loss for exact values)
        byte b = 100;
        short s = b;
        int i = s;
        long l = i;
        assert l == 100L : "widen.byte.to.long";

        // Narrowing conversions (explicit cast, potential data loss)
        
        // int to byte (keeps lower 8 bits, sign extends)
        assert ((byte) 127) == 127 : "narrow.i2b.max";
        assert ((byte) 128) == -128 : "narrow.i2b.wrap.128";
        assert ((byte) 255) == -1 : "narrow.i2b.wrap.255";
        assert ((byte) 256) == 0 : "narrow.i2b.wrap.256";
        assert ((byte) -129) == 127 : "narrow.i2b.wrap.neg";

        // int to short (keeps lower 16 bits, sign extends)
        assert ((short) 32767) == 32767 : "narrow.i2s.max";
        assert ((short) 32768) == -32768 : "narrow.i2s.wrap.32768";
        assert ((short) 65535) == -1 : "narrow.i2s.wrap.65535";
        assert ((short) 65536) == 0 : "narrow.i2s.wrap.65536";
        assert ((short) -32769) == 32767 : "narrow.i2s.wrap.neg";

        // int to char (keeps lower 16 bits, unsigned interpretation)
        assert ((char) 0) == '\u0000' : "narrow.i2c.zero";
        assert ((char) 65535) == '\uFFFF' : "narrow.i2c.max";
        assert ((char) 65536) == '\u0000' : "narrow.i2c.wrap.65536";
        assert ((char) -1) == '\uFFFF' : "narrow.i2c.neg";
        assert ((char) -32768) == '\u8000' : "narrow.i2c.neg.mid";

        // long to int (keeps lower 32 bits)
        assert ((int) 2147483647L) == 2147483647 : "narrow.l2i.max";
        assert ((int) 2147483648L) == -2147483648 : "narrow.l2i.wrap";
        assert ((int) 4294967295L) == -1 : "narrow.l2i.wrap.max";
        assert ((int) 4294967296L) == 0 : "narrow.l2i.wrap.cycle";

        // char to byte (keeps lower 8 bits, sign extends)
        assert ((byte) 'A') == 65 : "narrow.c2b.ascii";
        assert ((byte) '\u007F') == 127 : "narrow.c2b.max.positive";
        assert ((byte) '\u0080') == -128 : "narrow.c2b.wrap";
        assert ((byte) '\u00FF') == -1 : "narrow.c2b.255";

        // char to short (keeps all 16 bits, but reinterprets as signed)
        assert ((short) '\u0000') == 0 : "narrow.c2s.zero";
        assert ((short) '\u7FFF') == 32767 : "narrow.c2s.max.positive";
        assert ((short) '\u8000') == -32768 : "narrow.c2s.negative";
        assert ((short) '\uFFFF') == -1 : "narrow.c2s.max";

        // short to char (keeps all 16 bits, but reinterprets as unsigned)
        assert ((char) (short) 0) == '\u0000' : "narrow.s2c.zero";
        assert ((char) (short) 32767) == '\u7FFF' : "narrow.s2c.max.positive";
        assert ((char) (short) -32768) == '\u8000' : "narrow.s2c.min";
        assert ((char) (short) -1) == '\uFFFF' : "narrow.s2c.neg";

        // byte to char (sign extends to int first, then converts to char)
        assert ((char) (byte) 65) == 'A' : "narrow.b2c.positive";
        assert ((char) (byte) -1) == '\uFFFF' : "narrow.b2c.neg";
        assert ((char) (byte) -128) == '\uFF80' : "narrow.b2c.min";

        // Cross-type arithmetic (promotes to common type)
        byte b1 = 100;
        short s1 = 200;
        int result1 = b1 + s1; // Both promoted to int
        assert result1 == 300 : "cross.byte.short.add";

        char c1 = 'A'; // 65
        int result2 = c1 + 10; // char promoted to int
        assert result2 == 75 : "cross.char.int.add";

        // Mixed signed/unsigned (char is unsigned, others signed)
        char c2 = '\uFFFF'; // 65535 as unsigned
        byte b2 = -1;         // -1 as signed
        // Both promoted to int: 65535 + (-1) = 65534
        assert (c2 + b2) == 65534 : "cross.char.unsigned.byte.signed";

        // Long conversions
        long l1 = 9223372036854775807L; // Long.MAX_VALUE
        assert ((int) l1) == -1 : "narrow.long.max.to.int";
        assert ((byte) l1) == -1 : "narrow.long.max.to.byte";

        // Boolean conversions (note: boolean cannot be cast to numeric types in Java)
        // This is a compile-time error, so we don't test it

        System.out.println("All conversion assertions passed.");
    }
}
