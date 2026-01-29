package variables.array_components.regular.loop_access;

public class LoopAccessOkMain {
    public static void main(String[] args) {
        int[] arr = new int[10];
        
        // Write in loop
        for (int i = 0; i < 10; i++) {
            arr[i] = i * 10;
        }
        
        // Read and verify
        for (int i = 0; i < 10; i++) {
            assert arr[i] == i * 10 : "loop.rw";
        }
        
        // Sum elements
        int sum = 0;
        for (int i = 0; i < arr.length; i++) {
            sum += arr[i];
        }
        assert sum == 450 : "loop.sum"; // 0+10+20+...+90 = 450
        
        // Overwrite all
        for (int i = 0; i < arr.length; i++) {
            arr[i] = 1;
        }
        sum = 0;
        for (int i = 0; i < arr.length; i++) {
            sum += arr[i];
        }
        assert sum == 10 : "loop.overwrite";
        
        System.out.println("Loop access test passed.");
    }
}
