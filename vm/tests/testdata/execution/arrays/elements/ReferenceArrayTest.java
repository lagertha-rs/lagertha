// @test feature = "execution.arrays.reference-elements"
// @test description = "Verifies compatible references, null, identity, aliases, and array-reference reassignment."
// @test category = "success"

package execution.arrays.elements;

public class ReferenceArrayTest {
    public static void main(String[] args) {
        ArrayValue first = new ArrayValue(runtime(7));
        ArrayValue second = new ArrayValue(11);
        Object[] objects = new Object[3];
        objects[0] = first;
        objects[1] = null;
        objects[2] = first;

        assert objects[0] == first : "reference.object.identity";
        assert objects[1] == null : "reference.null";
        assert objects[0] == objects[2] : "reference.component.alias";

        ArrayValue[] values = new ArrayValue[2];
        values[0] = first;
        values[1] = second;
        assert values[0].number == 7 : "reference.narrow.component";

        ArrayValue[] alias = values;
        alias[0].number = 19;
        assert values[0].number == 19 : "reference.array.alias";

        alias = new ArrayValue[] { second };
        assert alias != values : "reference.array.reassigned";
        assert values.length == 2 : "reference.original.preserved";
    }

    static int runtime(int value) {
        return value;
    }
}

class ArrayValue {
    int number;

    ArrayValue(int number) {
        this.number = number;
    }
}
