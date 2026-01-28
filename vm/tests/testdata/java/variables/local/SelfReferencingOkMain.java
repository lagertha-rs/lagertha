package variables.local.self_referencing;

public class SelfReferencingOkMain {
    public static void main(String[] args) {
        // x = x + 1
        int x = 10;
        x = x + 1;
        assert x == 11 : "self.add";

        // x = x * x
        x = 5;
        x = x * x;
        assert x == 25 : "self.mul";

        // x = x / 2
        x = 100;
        x = x / 2;
        assert x == 50 : "self.div";

        // x = x % 3
        x = 17;
        x = x % 3;
        assert x == 2 : "self.rem";

        // x = -x
        x = 42;
        x = -x;
        assert x == -42 : "self.neg";

        // x = ~x
        x = 0;
        x = ~x;
        assert x == -1 : "self.not";

        // Boolean self-reference
        boolean b = true;
        b = !b;
        assert b == false : "self.bool.not";
        b = !b;
        assert b == true : "self.bool.not2";

        // Array index self-reference
        int[] arr = {1, 2, 3, 4, 5};
        int idx = 0;
        idx = arr[idx]; // idx = arr[0] = 1
        assert idx == 1 : "self.arr.idx";

        System.out.println("All self-referencing tests passed.");
    }
}
