package variables.array_components.errors.null_row_access;

public class NullRowAccessErrMain {
    public static void main(String[] args) {
        // Jagged array with null row
        int[][] jagged = new int[3][];
        // jagged[0] is null - this should throw NullPointerException
        int x = jagged[0][0];
        System.out.println("Should not print: " + x);
    }
}
