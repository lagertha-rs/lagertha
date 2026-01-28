package variables.parameters.array_parameters;

public class ArrayParametersOkMain {
    public static void main(String[] args) {
        // int array
        int[] intArr = {1, 2, 3, 4, 5};
        assert sumIntArray(intArr) == 15 : "arr.int.sum";

        // Object array
        Object[] objArr = new Object[3];
        assert objArrayLength(objArr) == 3 : "arr.obj.len";

        // byte array
        byte[] byteArr = {10, 20, 30};
        assert sumByteArray(byteArr) == 60 : "arr.byte.sum";

        // String array
        String[] strArr = {"a", "b", "c"};
        assert strArrayLength(strArr) == 3 : "arr.str.len";

        // 2D array
        int[][] matrix = {{1, 2}, {3, 4}};
        assert sum2DArray(matrix) == 10 : "arr.2d.sum";

        System.out.println("Array parameters test passed.");
    }

    static int sumIntArray(int[] arr) {
        int sum = 0;
        for (int i = 0; i < arr.length; i++) {
            sum += arr[i];
        }
        return sum;
    }

    static int objArrayLength(Object[] arr) {
        return arr.length;
    }

    static int sumByteArray(byte[] arr) {
        int sum = 0;
        for (int i = 0; i < arr.length; i++) {
            sum += arr[i];
        }
        return sum;
    }

    static int strArrayLength(String[] arr) {
        return arr.length;
    }

    static int sum2DArray(int[][] arr) {
        int sum = 0;
        for (int i = 0; i < arr.length; i++) {
            for (int j = 0; j < arr[i].length; j++) {
                sum += arr[i][j];
            }
        }
        return sum;
    }
}
