package variables.array_components.regular.index_expr;

public class IndexExprOkMain {
    public static void main(String[] args) {
        int[] arr = new int[10];
        for (int i = 0; i < 10; i++) {
            arr[i] = i * 100;
        }
        
        // Variable index
        int idx = 3;
        assert arr[idx] == 300 : "idx.var";
        
        // Expression index
        assert arr[2 + 3] == 500 : "idx.expr";
        
        // Method call as index
        assert arr[getIndex()] == 500 : "idx.method";
        
        // Index from array element
        int[] indices = {0, 5, 9};
        assert arr[indices[0]] == 0 : "idx.arr.0";
        assert arr[indices[1]] == 500 : "idx.arr.1";
        assert arr[indices[2]] == 900 : "idx.arr.2";
        
        // Index with modulo
        assert arr[15 % 10] == 500 : "idx.mod";
        
        System.out.println("Index expression test passed.");
    }
    
    static int getIndex() {
        return 5;
    }
}
