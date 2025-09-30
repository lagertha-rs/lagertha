package classes.get_primitive_array_class_name;

public class GetPrimitiveArrayClassNameMain {
    public static void main(String[] args) {
        int[] a = new int[1];
        var name = a.getClass().getName();
    }
}



