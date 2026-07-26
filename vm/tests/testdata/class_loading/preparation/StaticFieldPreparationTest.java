// @test feature = "class-loading.preparation"
// @test description = "Verifies class and interface static fields expose prepared defaults before explicit initializers."
// @test category = "success"

package classloading.preparation;

public class StaticFieldPreparationTest {
    public static void main(String[] args) {
        assert !PreparedClass.observedBoolean : "preparation.class.boolean";
        assert PreparedClass.observedByte == 0 : "preparation.class.byte";
        assert PreparedClass.observedChar == 0 : "preparation.class.char";
        assert PreparedClass.observedShort == 0 : "preparation.class.short";
        assert PreparedClass.observedInt == 0 : "preparation.class.int";
        assert PreparedClass.observedLong == 0L : "preparation.class.long";
        assert PreparedClass.observedFloat == 0.0f : "preparation.class.float";
        assert PreparedClass.observedDouble == 0.0d : "preparation.class.double";
        assert PreparedClass.observedReference == null : "preparation.class.reference";
        assert PreparedClass.laterInt == 7 : "preparation.class.initialized.int";
        assert PreparedClass.laterReference != null : "preparation.class.initialized.reference";

        assert PreparedInterface.OBSERVED_INT == 0 : "preparation.interface.int";
        assert PreparedInterface.OBSERVED_REFERENCE == null : "preparation.interface.reference";
        assert PreparedInterface.LATER_INT == 11 : "preparation.interface.initialized.int";
        assert PreparedInterface.LATER_REFERENCE != null : "preparation.interface.initialized.reference";
    }
}

class PreparedClass {
    static boolean observedBoolean = PreparedClass.laterBoolean;
    static byte observedByte = PreparedClass.laterByte;
    static char observedChar = PreparedClass.laterChar;
    static short observedShort = PreparedClass.laterShort;
    static int observedInt = PreparedClass.laterInt;
    static long observedLong = PreparedClass.laterLong;
    static float observedFloat = PreparedClass.laterFloat;
    static double observedDouble = PreparedClass.laterDouble;
    static Object observedReference = PreparedClass.laterReference;

    static boolean laterBoolean = true;
    static byte laterByte = 1;
    static char laterChar = 1;
    static short laterShort = 1;
    static int laterInt = 7;
    static long laterLong = 1L;
    static float laterFloat = 1.0f;
    static double laterDouble = 1.0d;
    static Object laterReference = new Object();
}

interface PreparedInterface {
    int OBSERVED_INT = PreparedInterface.LATER_INT;
    Object OBSERVED_REFERENCE = PreparedInterface.LATER_REFERENCE;
    int LATER_INT = runtime(11);
    Object LATER_REFERENCE = new Object();

    static int runtime(int value) {
        return value;
    }
}
