package variables.static_fields.cross_class_access;

public class CrossClassAccessOkMain {
    public static void main(String[] args) {
        // Access other class's static field
        assert OtherClass.publicStatic == 999 : "crossclass.read";

        // Modify other class's static field
        OtherClass.publicStatic = 888;
        assert OtherClass.publicStatic == 888 : "crossclass.write";

        // Reset
        OtherClass.publicStatic = 999;

        // Access from method in another class
        assert CrossAccessHelper.readOtherStatic() == 999 : "crossclass.via.method";

        System.out.println("Cross-class access test passed.");
    }
}

class OtherClass {
    static int publicStatic = 999;
}

class CrossAccessHelper {
    static int readOtherStatic() {
        return OtherClass.publicStatic;
    }
}
