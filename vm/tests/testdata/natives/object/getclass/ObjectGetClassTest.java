// @test feature = "natives.object.get-class"
// @test description = "Verifies getClass returns and reuses the implementation mirror through concrete, Object, and interface references."
// @test category = "success"

package natives.object.getclass;

public class ObjectGetClassTest {
    public static void main(String[] args) {
        MirrorImplementation implementation = new MirrorImplementation();
        Object objectReference = implementation;
        MirrorMarker interfaceReference = implementation;

        Class<?> fromObject = objectReference.getClass();
        Class<?> expected = MirrorImplementation.class;
        assert fromObject == expected : "getclass.object.runtime";
        assert implementation.getClass() == expected : "getclass.concrete.runtime";
        assert interfaceReference.getClass() == expected : "getclass.interface.runtime";
        assert new MirrorImplementation().getClass() == expected : "getclass.mirror.reused";
        assert interfaceReference.getClass() != MirrorMarker.class : "getclass.not.interface";
    }
}

interface MirrorMarker {}
final class MirrorImplementation implements MirrorMarker {}
