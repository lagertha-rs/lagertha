package classes.abstract_classes.basic;

public class AbstractClassOkMain {
    public static void main(String[] args) {
        ConcreteClass obj = new ConcreteClass();
        
        assert obj.getValue() == 42 : "implemented abstract method";
        assert obj.getConcreteValue() == 100 : "concrete method";
        
        // Cannot instantiate abstract class directly
        // AbstractClass abs = new AbstractClass(); // compile error
        
        System.out.println("Abstract class tests passed.");
    }
}

abstract class AbstractClass {
    public abstract int getValue();
    
    public int getConcreteValue() {
        return 100;
    }
}

class ConcreteClass extends AbstractClass {
    public int getValue() {
        return 42;
    }
}