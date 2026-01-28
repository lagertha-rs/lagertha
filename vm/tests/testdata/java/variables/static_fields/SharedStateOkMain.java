package variables.static_fields.shared_state;

public class SharedStateOkMain {
    public static void main(String[] args) {
        // Reset counter
        SharedStateHolder.counter = 0;

        // Create multiple instances, each increments
        SharedStateHolder h1 = new SharedStateHolder();
        assert SharedStateHolder.counter == 1 : "shared.h1.ctor";

        SharedStateHolder h2 = new SharedStateHolder();
        assert SharedStateHolder.counter == 2 : "shared.h2.ctor";

        SharedStateHolder h3 = new SharedStateHolder();
        assert SharedStateHolder.counter == 3 : "shared.h3.ctor";

        // Static method modifies
        SharedStateHolder.increment();
        assert SharedStateHolder.counter == 4 : "shared.static.method";

        // Instance method sees static
        assert h1.getCounter() == 4 : "shared.h1.sees";
        assert h2.getCounter() == 4 : "shared.h2.sees";
        assert h3.getCounter() == 4 : "shared.h3.sees";

        // One instance modifies, all see
        h1.incrementViaInstance();
        assert h2.getCounter() == 5 : "shared.h2.sees.h1mod";
        assert h3.getCounter() == 5 : "shared.h3.sees.h1mod";

        System.out.println("Shared state test passed.");
    }
}

class SharedStateHolder {
    static int counter = 0;

    SharedStateHolder() {
        counter++;
    }

    static void increment() {
        counter++;
    }

    int getCounter() {
        return counter;
    }

    void incrementViaInstance() {
        counter++;
    }
}
