package variables.instance.null_field_access;

public class NullFieldAccessErrMain {
    int value = 42;

    public static void main(String[] args) {
        NullFieldAccessErrMain obj = null;
        
        // This should throw NullPointerException
        int x = obj.value;
        
        // Should not reach here
        System.out.println("Should not print: " + x);
    }
}
