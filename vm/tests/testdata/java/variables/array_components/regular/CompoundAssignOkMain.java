package variables.array_components.regular.compound_assign;

public class CompoundAssignOkMain {
    public static void main(String[] args) {
        int[] arr = new int[1];
        arr[0] = 10;
        
        arr[0] += 5;
        assert arr[0] == 15 : "+=";
        
        arr[0] -= 3;
        assert arr[0] == 12 : "-=";
        
        arr[0] *= 2;
        assert arr[0] == 24 : "*=";
        
        arr[0] /= 4;
        assert arr[0] == 6 : "/=";
        
        arr[0] = 17;
        arr[0] %= 5;
        assert arr[0] == 2 : "%=";
        
        arr[0] = 0b1111;
        arr[0] &= 0b1010;
        assert arr[0] == 0b1010 : "&=";
        
        arr[0] = 0b1010;
        arr[0] |= 0b0101;
        assert arr[0] == 0b1111 : "|=";
        
        arr[0] = 0b1111;
        arr[0] ^= 0b1010;
        assert arr[0] == 0b0101 : "^=";
        
        arr[0] = 1;
        arr[0] <<= 4;
        assert arr[0] == 16 : "<<=";
        
        arr[0] = 32;
        arr[0] >>= 2;
        assert arr[0] == 8 : ">>=";
        
        arr[0] = -8;
        arr[0] >>>= 2;
        assert arr[0] == 1073741822 : ">>>=";
        
        System.out.println("Compound assignment test passed.");
    }
}
