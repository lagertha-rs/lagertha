package variables.array_components.regular.arraycopy;

public class ArraycopyOkMain {
    public static void main(String[] args) {
        // Basic copy
        int[] src = {1, 2, 3, 4, 5};
        int[] dst = new int[5];
        System.arraycopy(src, 0, dst, 0, 5);
        
        assert dst[0] == 1 : "copy.0";
        assert dst[4] == 5 : "copy.4";
        
        // Partial copy
        int[] partial = new int[10];
        System.arraycopy(src, 1, partial, 3, 3); // copy src[1..3] to partial[3..5]
        assert partial[3] == 2 : "partial.3";
        assert partial[4] == 3 : "partial.4";
        assert partial[5] == 4 : "partial.5";
        assert partial[0] == 0 : "partial.0.unchanged";
        
        // Self-copy (overlapping)
        int[] self = {1, 2, 3, 4, 5};
        System.arraycopy(self, 0, self, 1, 4); // shift right
        assert self[0] == 1 : "self.0";
        assert self[1] == 1 : "self.1";
        assert self[4] == 4 : "self.4";
        
        System.out.println("Arraycopy test passed.");
    }
}
