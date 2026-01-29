package classes.inheritance.single;

public class SingleInheritanceOkMain {
    public static void main(String[] args) {
        Child child = new Child();
        
        // Inherited field
        assert child.parentField == 10 : "inherited field";
        
        // Overridden method
        assert child.getValue() == 200 : "overridden method";
        
        // Parent method still accessible via super
        assert child.getParentValue() == 100 : "parent method via super";
        
        // Polymorphism
        Parent poly = new Child();
        assert poly.getValue() == 200 : "polymorphic call";
        
        System.out.println("Single inheritance tests passed.");
    }
}

class Parent {
    protected int parentField = 10;
    
    public int getValue() {
        return 100;
    }
}

class Child extends Parent {
    public int getValue() {
        return 200;
    }
    
    public int getParentValue() {
        return super.getValue();
    }
}