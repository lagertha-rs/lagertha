package classes.default_fields_values;

public class InstanceDefaultValuesMain {
    static class ClassWithFields {
        // Reference types
        private String str;
        private Object obj;
        private int[] intArray;

        // Primitives
        private boolean boolField;
        private byte byteField;
        private short shortField;
        private char charField;
        private int intField;
        private long longField;
        private float floatField;
        private double doubleField;
    }

    public static void main(String[] args) {
        var classWithFields = new ClassWithFields();
    }
}

