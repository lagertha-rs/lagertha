package variables.local.compound_assignments;

public class CompoundAssignmentsOkMain {
    public static void main(String[] args) {
        int val;

        // +=
        val = 10;
        val += 5;
        assert val == 15 : "compound.add";

        // -=
        val = 10;
        val -= 3;
        assert val == 7 : "compound.sub";

        // *=
        val = 10;
        val *= 4;
        assert val == 40 : "compound.mul";

        // /=
        val = 20;
        val /= 4;
        assert val == 5 : "compound.div";

        // %=
        val = 17;
        val %= 5;
        assert val == 2 : "compound.rem";

        // &=
        val = 0b1111;
        val &= 0b1010;
        assert val == 0b1010 : "compound.and";

        // |=
        val = 0b1010;
        val |= 0b0101;
        assert val == 0b1111 : "compound.or";

        // ^=
        val = 0b1111;
        val ^= 0b1010;
        assert val == 0b0101 : "compound.xor";

        // <<=
        val = 1;
        val <<= 4;
        assert val == 16 : "compound.shl";

        // >>=
        val = 32;
        val >>= 2;
        assert val == 8 : "compound.shr";

        // >>>=
        val = -8;
        val >>>= 2;
        assert val == 1073741822 : "compound.ushr";

        // Compound on byte (widening/narrowing)
        byte byteVal = 10;
        byteVal += 5;
        assert byteVal == 15 : "compound.byte";

        // Compound on long
        long longVal = 1000000000L;
        longVal *= 2;
        assert longVal == 2000000000L : "compound.long";

        // Compound on float
        float floatVal = 1.5f;
        floatVal += 0.5f;
        assert floatVal == 2.0f : "compound.float";

        // Compound on double
        double doubleVal = 1.5;
        doubleVal *= 2.0;
        assert doubleVal == 3.0 : "compound.double";

        System.out.println("All compound assignment tests passed.");
    }
}
