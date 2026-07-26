// @test feature = "execution.fields.access"
// @test description = "Verifies instance, static, initialized, default, and inherited field access."
// @test category = "success"

package execution.objects.fields;

public class FieldAccessTest {
    public static void main(String[] args) {
        FieldChild object = new FieldChild();
        assert object.initialized == 42 : "instance.initialized";
        assert object.defaultValue == 0 : "instance.default";
        object.initialized = 100;
        object.defaultValue = 200;
        assert object.initialized == 100 : "instance.write.initialized";
        assert object.defaultValue == 200 : "instance.write.default";
        assert object.inherited == 10 : "instance.inherited";

        assert FieldChild.staticValue == 0 : "static.default";
        FieldChild.staticValue = 999;
        assert FieldChild.staticValue == 999 : "static.write";
    }
}

class FieldParent {
    int inherited = 10;
}

class FieldChild extends FieldParent {
    int initialized = 42;
    int defaultValue;
    static int staticValue;
}
