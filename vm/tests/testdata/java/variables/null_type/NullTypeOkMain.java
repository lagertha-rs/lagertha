package variables.null_type.basic;

public class NullTypeOkMain {
    public static void main(String[] args) {
        // Null assignment to reference
        Object obj = null;
        assert obj == null : "null equality";
        
        // Null can be cast to any reference type
        String str = (String) null;
        assert str == null : "cast null";
        
        // Null comparison
        Object obj2 = null;
        assert obj == obj2 : "null equality between variables";
        
        // Null cannot be dereferenced (would throw NullPointerException at runtime)
        // We'll test that in error test
        
        // Null in arrays
        Object[] array = new Object[3];
        assert array[0] == null : "array elements default to null";
        
        // Null assignment to array element
        array[1] = new Object();
        array[1] = null;
        assert array[1] == null : "can assign null to array element";
        
        // Null as method argument
        assert isNull(null) : "null argument";
        
        // Null return
        assert getNull() == null : "null return";
        
        System.out.println("Null type tests passed.");
    }
    
    static boolean isNull(Object o) {
        return o == null;
    }
    
    static Object getNull() {
        return null;
    }
}