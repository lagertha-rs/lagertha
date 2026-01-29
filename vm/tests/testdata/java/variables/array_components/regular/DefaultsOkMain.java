package variables.array_components.regular.defaults;

public class DefaultsOkMain {
    public static void main(String[] args) {
        // All primitive arrays default to zero/false
        boolean[] bools = new boolean[3];
        assert bools[0] == false : "default.bool";
        
        byte[] bytes = new byte[3];
        assert bytes[0] == 0 : "default.byte";
        
        char[] chars = new char[3];
        assert chars[0] == '\u0000' : "default.char";
        
        short[] shorts = new short[3];
        assert shorts[0] == 0 : "default.short";
        
        int[] ints = new int[3];
        assert ints[0] == 0 : "default.int";
        
        long[] longs = new long[3];
        assert longs[0] == 0L : "default.long";
        
        float[] floats = new float[3];
        assert floats[0] == 0.0f : "default.float";
        
        double[] doubles = new double[3];
        assert doubles[0] == 0.0 : "default.double";
        
        // Reference arrays default to null
        Object[] objs = new Object[3];
        assert objs[0] == null : "default.obj";
        
        String[] strs = new String[3];
        assert strs[0] == null : "default.str";
        
        System.out.println("Defaults test passed.");
    }
}
