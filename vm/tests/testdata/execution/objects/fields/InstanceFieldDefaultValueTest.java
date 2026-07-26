// @test feature = "execution.objects.instance-default-values"
// @test description = "Verifies primitive, reference, inherited, and per-object instance field defaults."
// @test category = "success"

package execution.objects.fields;

public class InstanceFieldDefaultValueTest {
    public static void main(String[] args) {
        DefaultChild first = new DefaultChild();
        DefaultChild second = new DefaultChild();

        assert !first.booleanValue : "default.boolean";
        assert first.byteValue == 0 : "default.byte";
        assert first.charValue == 0 : "default.char";
        assert first.shortValue == 0 : "default.short";
        assert first.intValue == 0 : "default.int";
        assert first.longValue == 0L : "default.long";
        assert first.floatValue == 0.0f : "default.float";
        assert first.doubleValue == 0.0d : "default.double";
        assert first.objectValue == null : "default.object";
        assert first.stringValue == null : "default.string";
        assert first.arrayValue == null : "default.array";
        assert first.inheritedInt == 0 : "default.inherited.int";
        assert first.inheritedReference == null : "default.inherited.reference";

        first.intValue = 9;
        first.objectValue = first;
        assert second.intValue == 0 : "default.independent.int";
        assert second.objectValue == null : "default.independent.reference";
    }
}

class DefaultParent {
    int inheritedInt;
    Object inheritedReference;
}

class DefaultChild extends DefaultParent {
    boolean booleanValue;
    byte byteValue;
    char charValue;
    short shortValue;
    int intValue;
    long longValue;
    float floatValue;
    double doubleValue;
    Object objectValue;
    String stringValue;
    int[] arrayValue;
}
