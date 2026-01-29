package variables.array_components.regular.reference_assign;

public class ReferenceAssignOkMain {
    public static void main(String[] args) {
        // Array variable assignment (reference copy)
        int[] arr1 = new int[]{1, 2, 3};
        int[] arr2 = arr1; // Same reference
        
        assert arr1 == arr2 : "same.ref";
        assert arr2[0] == 1 : "arr2.0";
        
        // Modify via arr2, visible in arr1
        arr2[0] = 999;
        assert arr1[0] == 999 : "shared.mod";
        
        // Reassign arr2 to new array
        arr2 = new int[]{10, 20, 30};
        assert arr1[0] == 999 : "arr1.unchanged";
        assert arr2[0] == 10 : "arr2.new";
        assert arr1 != arr2 : "diff.ref";
        
        // Null assignment
        arr2 = null;
        assert arr2 == null : "null";
        assert arr1 != null : "arr1.not.null";
        
        System.out.println("Reference assignment test passed.");
    }
}
