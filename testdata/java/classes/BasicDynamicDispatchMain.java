package classes.basic_dynamic_dispatch;

// Test that dynamic dispatch works correctly for methods defined on Object
// (here, hashCode). The expected output is the hash code of the java.lang.String
// "java.version", which is 560567564.
// This test is relevant because the implementation of dynamic dispatch
// in the JVM is different from the one in the native image (for java.lang.Object).

public class BasicDynamicDispatchMain {
    public static void main(String[] args) {
        Object o = "java.version";
        int hash = o.hashCode(); // should be 560567564
    }
}