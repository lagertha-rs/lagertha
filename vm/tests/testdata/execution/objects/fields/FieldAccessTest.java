// @test feature = "execution.fields.access"
// @test description = "Verifies primitive and reference fields, receiver storage, shared statics, inheritance, hiding, and null writes."
// @test category = "success"

package execution.objects.fields;

public class FieldAccessTest {
    public static void main(String[] args) {
        int value = runtime(7);
        FieldChild first = new FieldChild();
        FieldChild second = new FieldChild();

        first.booleanValue = true;
        first.byteValue = (byte) -128;
        first.charValue = Character.MAX_VALUE;
        first.shortValue = Short.MIN_VALUE;
        first.intValue = value;
        first.longValue = Long.MIN_VALUE;
        first.floatValue = -1.5f;
        first.doubleValue = 2.25d;
        first.referenceValue = second;
        first.arrayValue = new int[] { value };
        first.self = first;

        assert first.booleanValue : "instance.boolean";
        assert first.byteValue == -128 : "instance.byte";
        assert first.charValue == 65535 : "instance.char";
        assert first.shortValue == -32768 : "instance.short";
        assert first.intValue == 7 : "instance.int";
        assert first.longValue == Long.MIN_VALUE : "instance.long";
        assert first.floatValue == -1.5f : "instance.float";
        assert first.doubleValue == 2.25d : "instance.double";
        assert first.referenceValue == second : "instance.reference";
        assert first.arrayValue[0] == 7 : "instance.array";
        assert first.self == first : "instance.self";
        assert second.intValue == 0 : "instance.independent";

        second.intValue = first.intValue + 4;
        assert second.intValue == 11 : "instance.cross.object";
        assert first.inherited == 10 : "instance.inherited";
        assert first.hidden == 30 : "instance.hidden.child";
        assert ((FieldParent) first).hidden == 20 : "instance.hidden.parent";

        FieldChild.staticInt = value;
        FieldChild.staticLong = Long.MAX_VALUE;
        FieldChild.staticDouble = -4.5d;
        FieldChild.staticReference = first;
        assert FieldChild.staticInt == 7 : "static.int";
        assert FieldChild.staticLong == Long.MAX_VALUE : "static.long";
        assert FieldChild.staticDouble == -4.5d : "static.double";
        assert FieldChild.staticReference == first : "static.reference";
        assert second.staticInt == 7 : "static.shared";
        assert FieldChild.staticHidden == 50 : "static.hidden.child";
        assert FieldParent.staticHidden == 40 : "static.hidden.parent";

        FieldChild nullReceiver = null;
        try {
            nullReceiver.intValue = 1;
            assert false : "instance.null.write.missing.exception";
        } catch (NullPointerException expected) {
            assert nullReceiver == null : "instance.null.write";
        }
    }

    static int runtime(int value) {
        return value;
    }
}

class FieldParent {
    int inherited = 10;
    int hidden = 20;
    static int staticHidden = 40;
}

class FieldChild extends FieldParent {
    boolean booleanValue;
    byte byteValue;
    char charValue;
    short shortValue;
    int intValue;
    long longValue;
    float floatValue;
    double doubleValue;
    FieldChild referenceValue;
    int[] arrayValue;
    FieldChild self;
    int hidden = 30;

    static int staticInt;
    static long staticLong;
    static double staticDouble;
    static FieldChild staticReference;
    static int staticHidden = 50;
}
