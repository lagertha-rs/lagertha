// @test feature = "execution.integer.conversions"
// @test description = "Verifies narrowing integers to byte, short, and char values."
// @test category = "success"

package execution.integer.conversions;

public class NarrowingConversionTest {
    public static void main(String[] args) {
        int byteInput = 256;
        int byteSignBoundary = 128;
        int byteNegativeWrap = -129;
        int shortInput = 65535;
        int shortSignBoundary = 32768;
        int shortNegativeWrap = -32769;
        int charInput = -1;
        int charNegativeMidpoint = -32768;
        int charWrapInput = 65536;
        assert (byte) byteInput == 0 : "cast.i2b.256";
        assert (byte) byteSignBoundary == -128 : "cast.i2b.128";
        assert (byte) byteNegativeWrap == 127 : "cast.i2b.-129";
        assert (short) shortInput == -1 : "cast.i2s.65535";
        assert (short) shortSignBoundary == -32768 : "cast.i2s.32768";
        assert (short) shortNegativeWrap == 32767 : "cast.i2s.-32769";
        assert (char) charInput == 65535 : "cast.i2c.minus1";
        assert (char) charNegativeMidpoint == 32768 : "cast.i2c.-32768";
        assert (char) charWrapInput == 0 : "cast.i2c.65536";

        char maximum = (char) shortInput;
        assert (int) maximum + 1 == 65536 : "cast.char.promote";
        char byteHigh = (char) 255;
        char byteWrap = (char) 256;
        assert (byte) byteHigh == -1 : "cast.c2b.high";
        assert (byte) byteWrap == 0 : "cast.c2b.wrap";
        assert (short) maximum == -1 : "cast.c2s.max";
    }
}
