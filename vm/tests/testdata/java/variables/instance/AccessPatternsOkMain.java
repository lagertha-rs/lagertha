package variables.instance.access_patterns;

public class AccessPatternsOkMain {
    public static void main(String[] args) {
        AccessPatternHolder holder = new AccessPatternHolder();

        // Access via method that uses this.field
        assert holder.getValueViaThis() == 100 : "access.this.get";
        holder.setValueViaThis(200);
        assert holder.value == 200 : "access.this.set";

        // Access via method without this
        assert holder.getValueDirect() == 200 : "access.direct.get";
        holder.setValueDirect(300);
        assert holder.value == 300 : "access.direct.set";

        // Field initialized with expression
        ExpressionInitHolder exprHolder = new ExpressionInitHolder();
        assert exprHolder.computed == 15 : "access.expr.init";
        assert exprHolder.fromMethod == 42 : "access.method.init";

        // Field initialized in constructor
        ConstructorInitHolder ctorHolder = new ConstructorInitHolder(999);
        assert ctorHolder.value == 999 : "access.ctor.init";

        System.out.println("Access patterns test passed.");
    }
}

class AccessPatternHolder {
    int value = 100;

    int getValueViaThis() {
        return this.value;
    }

    void setValueViaThis(int v) {
        this.value = v;
    }

    int getValueDirect() {
        return value;
    }

    void setValueDirect(int v) {
        value = v;
    }
}

class ExpressionInitHolder {
    int computed = 5 + 10;
    int fromMethod = computeValue();

    static int computeValue() {
        return 42;
    }
}

class ConstructorInitHolder {
    int value;

    ConstructorInitHolder(int v) {
        this.value = v;
    }
}
