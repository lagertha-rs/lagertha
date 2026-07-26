// @test feature = "execution.integer.conversions"
// @test description = "Verifies narrowing integers to byte, short, and char values."
// @test category = "success"

package execution.integer.conversions;

public class NarrowingConversionTest {
    public static void main(String[] args) {
        int byteInput = 256;
        int shortInput = 65535;
        int charInput = -1;
        assert (byte) byteInput == 0 : "cast.i2b.256";
        assert (short) shortInput == -1 : "cast.i2s.65535";
        assert (char) charInput == 65535 : "cast.i2c.minus1";
        char maximum = (char) shortInput;
        assert (int) maximum + 1 == 65536 : "cast.char.promote";
    }
}
