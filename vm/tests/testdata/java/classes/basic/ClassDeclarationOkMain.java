package classes.basic.class_declaration;

public class ClassDeclarationOkMain {
    public static void main(String[] args) {
        // Test basic class instantiation and field access
        SimpleClass obj = new SimpleClass();
        
        assert obj.value == 42 : "default field value";
        obj.value = 100;
        assert obj.value == 100 : "field assignment";
        
        assert obj.getValue() == 100 : "method access";
        obj.setValue(200);
        assert obj.getValue() == 200 : "method modification";
        
        // Static field and method
        assert SimpleClass.staticField == 0 : "static field default";
        SimpleClass.staticField = 999;
        assert SimpleClass.staticField == 999 : "static field assignment";
        assert SimpleClass.getStaticField() == 999 : "static method access";
        
        System.out.println("Basic class tests passed.");
    }
}

class SimpleClass {
    public int value = 42;
    public static int staticField = 0;
    
    public int getValue() {
        return value;
    }
    
    public void setValue(int v) {
        value = v;
    }
    
    public static int getStaticField() {
        return staticField;
    }
}