// @test feature = "execution.references.casting"
// @test description = "Verifies checkcast accepts null for class, interface, and array targets."
// @test category = "success"

package execution.references.casting;

public class NullCastTest {
    public static void main(String[] args) {
        Object value = runtimeNull();
        String string = (String) value;
        NullCastMarker marker = (NullCastMarker) value;
        int[] array = (int[]) value;

        assert string == null : "casting.null.class";
        assert marker == null : "casting.null.interface";
        assert array == null : "casting.null.array";
    }

    static Object runtimeNull() {
        return null;
    }
}

interface NullCastMarker {}
