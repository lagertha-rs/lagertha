package variables.parameters.constructor_parameters;

public class ConstructorParametersOkMain {
    public static void main(String[] args) {
        // Single parameter constructor
        Holder h1 = new Holder(42);
        assert h1.value == 42 : "ctor.single";

        // Multiple parameter constructor
        Point p = new Point(10, 20);
        assert p.x == 10 : "ctor.multi.x";
        assert p.y == 20 : "ctor.multi.y";

        // Constructor with all primitive types
        AllTypesHolder all = new AllTypesHolder(
            true, (byte) 1, 'X', (short) 100, 1000, 10000L, 1.5f, 2.5
        );
        assert all.boolVal == true : "ctor.all.bool";
        assert all.byteVal == 1 : "ctor.all.byte";
        assert all.charVal == 'X' : "ctor.all.char";
        assert all.shortVal == 100 : "ctor.all.short";
        assert all.intVal == 1000 : "ctor.all.int";
        assert all.longVal == 10000L : "ctor.all.long";
        assert all.floatVal == 1.5f : "ctor.all.float";
        assert all.doubleVal == 2.5 : "ctor.all.double";

        System.out.println("Constructor parameters test passed.");
    }
}

class Holder {
    int value;
    Holder(int v) { this.value = v; }
}

class Point {
    int x, y;
    Point(int x, int y) {
        this.x = x;
        this.y = y;
    }
}

class AllTypesHolder {
    boolean boolVal;
    byte byteVal;
    char charVal;
    short shortVal;
    int intVal;
    long longVal;
    float floatVal;
    double doubleVal;

    AllTypesHolder(boolean b, byte by, char c, short s, int i, long l, float f, double d) {
        this.boolVal = b;
        this.byteVal = by;
        this.charVal = c;
        this.shortVal = s;
        this.intVal = i;
        this.longVal = l;
        this.floatVal = f;
        this.doubleVal = d;
    }
}
