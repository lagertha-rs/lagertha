package variables.local.block_scoping;

public class BlockScopingOkMain {
    public static void main(String[] args) {
        int outer = 1;
        assert outer == 1 : "outer.before.block";

        {
            // Inner block - can declare new variables
            int outer2 = 2;
            int inner = 10;
            assert outer == 1 : "outer.visible.in.block";
            assert inner == 10 : "inner.in.block";
            assert outer2 == 2 : "outer2.in.block";
        }
        // inner is not visible here
        assert outer == 1 : "outer.after.block";

        // Nested blocks
        {
            int level1 = 100;
            {
                int level2 = 200;
                {
                    int level3 = 300;
                    assert level1 == 100 : "nested.level1";
                    assert level2 == 200 : "nested.level2";
                    assert level3 == 300 : "nested.level3";
                }
            }
        }

        // Reuse variable name after scope ends
        {
            int reused = 111;
            assert reused == 111 : "reused.first";
        }
        {
            int reused = 222;
            assert reused == 222 : "reused.second";
        }

        System.out.println("All block scoping tests passed.");
    }
}
