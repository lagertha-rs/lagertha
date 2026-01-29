package variables.array_components.regular.string_array;

public class StringArrayOkMain {
    public static void main(String[] args) {
        String[] arr = new String[4];
        arr[0] = "hello";
        arr[1] = "world";
        arr[2] = "";
        arr[3] = null;
        
        assert arr[0] == "hello" : "str.0";
        assert arr[1] == "world" : "str.1";
        assert arr[2] == "" : "str.empty";
        assert arr[3] == null : "str.null";
        assert arr.length == 4 : "str.length";
        
        // String interning - same literal is same reference
        arr[3] = "hello";
        assert arr[0] == arr[3] : "str.intern";
        
        System.out.println("String array test passed.");
    }
}
