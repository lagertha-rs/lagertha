package variables.defaults.instance_field_defaults;

public class InstanceFieldDefaultsOkMain {
    public static void main(String[] args) {
        InstanceFieldDefaults obj = new InstanceFieldDefaults();

        // boolean defaults to false
        assert obj.boolField == false : "inst.bool.default";

        // byte defaults to 0
        assert obj.byteField == 0 : "inst.byte.default";

        // short defaults to 0
        assert obj.shortField == 0 : "inst.short.default";

        // char defaults to '\u0000' (0)
        assert obj.charField == '\u0000' : "inst.char.default";
        assert obj.charField == 0 : "inst.char.default.numeric";

        // int defaults to 0
        assert obj.intField == 0 : "inst.int.default";

        // long defaults to 0L
        assert obj.longField == 0L : "inst.long.default";

        // float defaults to 0.0f
        assert obj.floatField == 0.0f : "inst.float.default";

        // double defaults to 0.0
        assert obj.doubleField == 0.0 : "inst.double.default";

        // Object defaults to null
        assert obj.objectField == null : "inst.object.default";

        // String defaults to null
        assert obj.stringField == null : "inst.string.default";

        // Array defaults to null
        assert obj.arrayField == null : "inst.array.default";

        // Custom class defaults to null
        assert obj.holderField == null : "inst.holder.default";

        System.out.println("Instance field defaults test passed.");
    }
}

class InstanceFieldDefaults {
    boolean boolField;
    byte byteField;
    short shortField;
    char charField;
    int intField;
    long longField;
    float floatField;
    double doubleField;
    Object objectField;
    String stringField;
    int[] arrayField;
    Holder holderField;
}

class Holder {
    int value;
}
