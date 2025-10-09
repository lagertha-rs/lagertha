package abstracts.basic_abstract;

public class BasicAbstractMain {
    static abstract class Animal {
        abstract String get_sound();
    }

    static class Cat extends Animal {
        @Override
        String get_sound() {
            return "Meow";
        }
    }

    public static void main(String[] args) {
        Animal abstractInstance = new Cat();
        Cat childInstance = new Cat();
        var abstractInstanceRes = abstractInstance.get_sound();
        var childInstanceRes = childInstance.get_sound();
    }
}
