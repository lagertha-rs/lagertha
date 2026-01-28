package variables.parameters.pass_by_reference;

public class PassByReferenceOkMain {
    public static void main(String[] args) {
        // Modifying object STATE via reference DOES affect caller
        Holder holder = new Holder(10);
        modifyHolderState(holder);
        assert holder.value == 999 : "pbr.obj.state.changed";

        // But reassigning parameter doesn't affect caller's reference
        Holder holder2 = new Holder(20);
        reassignHolder(holder2);
        assert holder2.value == 20 : "pbr.obj.ref.unchanged";

        // String is immutable but reassignment doesn't affect caller
        String str = "original";
        reassignString(str);
        assert str == "original" : "pbr.string.unchanged";

        // Modifying array CONTENTS via reference DOES affect caller
        int[] arr = {1, 2, 3};
        modifyArrayContents(arr);
        assert arr[0] == 999 : "pbr.arr.contents.changed";

        // But reassigning array parameter doesn't affect caller
        int[] arr2 = {1, 2, 3};
        reassignArray(arr2);
        assert arr2[0] == 1 : "pbr.arr.ref.unchanged";
        assert arr2.length == 3 : "pbr.arr.len.unchanged";

        System.out.println("Pass by reference test passed.");
    }

    static void modifyHolderState(Holder h) {
        h.value = 999; // Modifies the object the reference points to
    }

    static void reassignHolder(Holder h) {
        h = new Holder(999); // Reassigns local reference, doesn't affect caller
    }

    static void reassignString(String s) {
        s = "modified"; // Reassigns local reference
    }

    static void modifyArrayContents(int[] arr) {
        arr[0] = 999; // Modifies the array the reference points to
    }

    static void reassignArray(int[] arr) {
        arr = new int[]{100, 200, 300}; // Reassigns local reference
    }
}

class Holder {
    int value;
    Holder(int v) { this.value = v; }
}
