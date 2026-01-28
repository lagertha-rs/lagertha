package variables.parameters.instance_method_parameters;

public class InstanceMethodParametersOkMain {
    public static void main(String[] args) {
        InstanceMethods im = new InstanceMethods(100);

        // Instance method with primitive
        assert im.addToValue(50) == 150 : "instance.add";

        // Instance method with object
        Holder h = new Holder(25);
        assert im.addHolderToValue(h) == 125 : "instance.holder";

        // Instance method modifying instance state via parameter
        im.setValueFrom(new Holder(999));
        assert im.value == 999 : "instance.setfrom";

        System.out.println("Instance method parameters test passed.");
    }
}

class Holder {
    int value;
    Holder(int v) { this.value = v; }
}

class InstanceMethods {
    int value;

    InstanceMethods(int v) {
        this.value = v;
    }

    int addToValue(int x) {
        return this.value + x;
    }

    int addHolderToValue(Holder h) {
        return this.value + h.value;
    }

    void setValueFrom(Holder h) {
        this.value = h.value;
    }
}
