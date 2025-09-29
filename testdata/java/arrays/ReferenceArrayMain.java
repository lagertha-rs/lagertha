package arrays.reference_array;

public class ReferenceArrayMain {
    static class ReferenceClass{
        int value;
        ReferenceClass(int value) {
            this.value = value;
        }
    }
    public static void main(String[] args) {
        var ref1 = new ReferenceClass(1);
        var ref2 = new ReferenceClass(2);
        var ref3 = new ReferenceClass(3);
        var refArray = new ReferenceClass[]{ref1, ref2, ref3};
        var arr_len = refArray.length;
    }
}

