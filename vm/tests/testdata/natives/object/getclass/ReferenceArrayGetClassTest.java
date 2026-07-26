// @test feature = "natives.object.get-class"
// @test description = "Verifies one-dimensional reference arrays reuse one mirror across getClass calls, literals, and instances."
// @test category = "success"

package natives.object.getclass;

public class ReferenceArrayGetClassTest {
    public static void main(String[] args) {
        Object[] first = new Object[2];
        Class<?> fromObject = first.getClass();
        Class<?> expected = Object[].class;

        assert fromObject == expected : "getclass.reference.array";
        assert first.getClass() == fromObject : "getclass.reference.array.repeat";
        assert new Object[0].getClass() == expected : "getclass.reference.array.reused";
    }
}
