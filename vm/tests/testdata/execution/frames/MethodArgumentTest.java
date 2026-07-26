// @test feature = "execution.frames.method-arguments"
// @test description = "Verifies argument slot order, receivers, wide values, null, arrays, and value-copy semantics."
// @test category = "success"

package execution.frames;

public class MethodArgumentTest {
    public static void main(String[] args) {
        ArgumentBox object = new ArgumentBox(7);
        int[] array = new int[] { 10, 20 };
        verifyStatic(1, 2L, 3.5f, 4.5d, object, array, null);

        ArgumentReceiver receiver = new ArgumentReceiver(10);
        assert receiver.combine(5, object) == 22 : "arguments.instance.receiver";

        ArgumentHolder holder = new ArgumentHolder(1, 2L, 3.5f, 4.5d, object, array);
        assert holder.verify() : "arguments.constructor.layout";

        int primitive = 7;
        reassignPrimitive(primitive);
        assert primitive == 7 : "arguments.primitive.copy";

        mutateAndReassign(object);
        assert object.value == 99 : "arguments.reference.alias";
    }

    static void verifyStatic(
            int integer,
            long longValue,
            float floating,
            double doubleValue,
            ArgumentBox object,
            int[] array,
            Object nullable) {
        assert integer == 1 : "arguments.static.int";
        assert longValue == 2L : "arguments.static.long";
        assert floating == 3.5f : "arguments.static.float";
        assert doubleValue == 4.5d : "arguments.static.double";
        assert object.value == 7 : "arguments.static.reference";
        assert array[1] == 20 : "arguments.static.array";
        assert nullable == null : "arguments.static.null";
    }

    static void reassignPrimitive(int value) {
        value = 100;
        assert value == 100 : "arguments.primitive.callee";
    }

    static void mutateAndReassign(ArgumentBox value) {
        value.value = 99;
        value = new ArgumentBox(123);
        assert value.value == 123 : "arguments.reference.reassigned";
    }
}

class ArgumentBox {
    int value;

    ArgumentBox(int value) {
        this.value = value;
    }
}

class ArgumentReceiver {
    int base;

    ArgumentReceiver(int base) {
        this.base = base;
    }

    int combine(int increment, ArgumentBox box) {
        return base + increment + box.value;
    }
}

class ArgumentHolder {
    int integer;
    long longValue;
    float floating;
    double doubleValue;
    ArgumentBox object;
    int[] array;

    ArgumentHolder(
            int integer,
            long longValue,
            float floating,
            double doubleValue,
            ArgumentBox object,
            int[] array) {
        this.integer = integer;
        this.longValue = longValue;
        this.floating = floating;
        this.doubleValue = doubleValue;
        this.object = object;
        this.array = array;
    }

    boolean verify() {
        return integer == 1
                && longValue == 2L
                && floating == 3.5f
                && doubleValue == 4.5d
                && object.value == 7
                && array[1] == 20;
    }
}
