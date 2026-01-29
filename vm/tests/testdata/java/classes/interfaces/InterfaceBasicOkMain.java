package classes.interfaces.basic;

public class InterfaceBasicOkMain {
    public static void main(String[] args) {
        MyInterface obj = new InterfaceImpl();
        
        assert obj.getValue() == 42 : "interface method call";
        assert obj.getDefaultValue() == 100 : "default method";
        assert MyInterface.getStaticValue() == 999 : "static method";
        
        System.out.println("Interface tests passed.");
    }
}

interface MyInterface {
    int getValue();
    
    default int getDefaultValue() {
        return 100;
    }
    
    static int getStaticValue() {
        return 999;
    }
}

class InterfaceImpl implements MyInterface {
    public int getValue() {
        return 42;
    }
}