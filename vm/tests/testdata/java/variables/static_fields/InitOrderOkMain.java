package variables.static_fields.init_order;

public class InitOrderOkMain {
    public static void main(String[] args) {
        // Fields should be initialized in declaration order
        assert InitOrderHolder.first == 1 : "initorder.first";
        assert InitOrderHolder.second == 2 : "initorder.second";
        assert InitOrderHolder.third == 3 : "initorder.third";
        
        // Computed fields depend on previous fields
        assert InitOrderHolder.firstPlusSecond == 3 : "initorder.computed1";
        assert InitOrderHolder.allSum == 6 : "initorder.computed2";

        System.out.println("Static initialization order test passed.");
    }
}

class InitOrderHolder {
    static int first = 1;
    static int second = 2;
    static int third = 3;
    static int firstPlusSecond = first + second;
    static int allSum = first + second + third;
}
