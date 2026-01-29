package variables.array_components.regular.object_array;

public class ObjectArrayOkMain {
    public static void main(String[] args) {
        Object[] arr = new Object[3];
        Object o1 = new Object();
        Object o2 = new Object();
        
        arr[0] = o1;
        arr[1] = o2;
        arr[2] = null;
        
        assert arr[0] == o1 : "obj.0";
        assert arr[1] == o2 : "obj.1";
        assert arr[2] == null : "obj.null";
        assert arr.length == 3 : "obj.length";
        
        // Same object in multiple slots
        arr[2] = o1;
        assert arr[0] == arr[2] : "obj.same";
        
        System.out.println("Object array test passed.");
    }
}
