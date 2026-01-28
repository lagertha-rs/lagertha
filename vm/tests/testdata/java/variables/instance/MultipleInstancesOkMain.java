package variables.instance.multiple_instances;

public class MultipleInstancesOkMain {
    public static void main(String[] args) {
        SimpleHolder h1 = new SimpleHolder();
        SimpleHolder h2 = new SimpleHolder();
        SimpleHolder h3 = new SimpleHolder();

        // Each instance starts with same default
        assert h1.value == 0 : "multi.h1.init";
        assert h2.value == 0 : "multi.h2.init";
        assert h3.value == 0 : "multi.h3.init";

        // Modify one, others unchanged
        h1.value = 100;
        assert h1.value == 100 : "multi.h1.mod";
        assert h2.value == 0 : "multi.h2.unchanged";
        assert h3.value == 0 : "multi.h3.unchanged";

        // Modify another
        h2.value = 200;
        assert h1.value == 100 : "multi.h1.still";
        assert h2.value == 200 : "multi.h2.mod";
        assert h3.value == 0 : "multi.h3.still";

        // All different
        h3.value = 300;
        assert h1.value == 100 : "multi.h1.final";
        assert h2.value == 200 : "multi.h2.final";
        assert h3.value == 300 : "multi.h3.final";

        // Reference fields - independence
        RefHolder rh1 = new RefHolder();
        RefHolder rh2 = new RefHolder();
        
        rh1.arr = new int[]{1, 2, 3};
        rh2.arr = new int[]{4, 5, 6};
        
        assert rh1.arr[0] == 1 : "multi.ref.h1";
        assert rh2.arr[0] == 4 : "multi.ref.h2";

        System.out.println("Multiple instances test passed.");
    }
}

class SimpleHolder {
    int value;
}

class RefHolder {
    int[] arr;
}
