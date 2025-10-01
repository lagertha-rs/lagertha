package classes.class_is_primitive;

public class ClassIsPrimitiveMain {
    public static void main(String[] args) {
        Class<?> intClass = Integer.TYPE;
        var a = intClass.isPrimitive();
    }
}