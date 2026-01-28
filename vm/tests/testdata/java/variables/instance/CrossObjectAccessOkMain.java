package variables.instance.cross_object_access;

public class CrossObjectAccessOkMain {
    public static void main(String[] args) {
        CrossAccessHolder c1 = new CrossAccessHolder(10);
        CrossAccessHolder c2 = new CrossAccessHolder(20);

        // c1 reads c2's field
        assert c1.readOther(c2) == 20 : "cross.read";

        // c1 writes c2's field
        c1.writeOther(c2, 999);
        assert c2.value == 999 : "cross.write";

        // c1 swaps with c2
        c1.value = 111;
        c2.value = 222;
        c1.swapWith(c2);
        assert c1.value == 222 : "cross.swap.c1";
        assert c2.value == 111 : "cross.swap.c2";

        // Access within method
        CrossAccessHolder c3 = new CrossAccessHolder(50);
        assert c3.addOther(c1) == 272 : "cross.add"; // 50 + 222

        System.out.println("Cross-object access test passed.");
    }
}

class CrossAccessHolder {
    int value;

    CrossAccessHolder(int v) {
        this.value = v;
    }

    int readOther(CrossAccessHolder other) {
        return other.value;
    }

    void writeOther(CrossAccessHolder other, int v) {
        other.value = v;
    }

    void swapWith(CrossAccessHolder other) {
        int temp = this.value;
        this.value = other.value;
        other.value = temp;
    }

    int addOther(CrossAccessHolder other) {
        return this.value + other.value;
    }
}
