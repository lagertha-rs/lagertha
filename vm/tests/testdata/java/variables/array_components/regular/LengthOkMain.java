package variables.array_components.regular.length;

public class LengthOkMain {
    public static void main(String[] args) {
        int[] arr0 = new int[0];
        assert arr0.length == 0 : "len.0";
        
        int[] arr1 = new int[1];
        assert arr1.length == 1 : "len.1";
        
        int[] arr5 = new int[5];
        assert arr5.length == 5 : "len.5";
        
        int[] arr100 = new int[100];
        assert arr100.length == 100 : "len.100";
        
        // Length from variable
        int size = 10;
        int[] arrVar = new int[size];
        assert arrVar.length == 10 : "len.var";
        
        // Length from expression
        int[] arrExpr = new int[2 + 3];
        assert arrExpr.length == 5 : "len.expr";
        
        System.out.println("Length test passed.");
    }
}
