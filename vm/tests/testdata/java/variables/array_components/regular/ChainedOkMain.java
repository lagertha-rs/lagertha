package variables.array_components.regular.chained;

public class ChainedOkMain {
    public static void main(String[] args) {
        int[] arr = new int[10];
        
        // arr[arr[0]]
        arr[0] = 5;
        arr[5] = 999;
        assert arr[arr[0]] == 999 : "chain.simple";
        
        // arr[arr[arr[0]]]
        arr[0] = 1;
        arr[1] = 2;
        arr[2] = 777;
        assert arr[arr[arr[0]]] == 777 : "chain.deep";
        
        // Chain with modification
        arr[0] = 3;
        arr[3] = 0;
        arr[arr[arr[0]]] = 123; // arr[arr[3]] = arr[0] = 123
        assert arr[0] == 123 : "chain.mod";
        
        System.out.println("Chained access test passed.");
    }
}
