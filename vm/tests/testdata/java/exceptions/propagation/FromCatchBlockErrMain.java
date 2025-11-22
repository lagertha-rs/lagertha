package exceptions.propagation.from_catch_block;

public class FromCatchBlockErrMain {
    public static void main(String[] args) {
        try {
            throw new NullPointerException("Original");
        } catch (Throwable e) {
            throw new IllegalArgumentException("New exception in catch");
        }
    }
}