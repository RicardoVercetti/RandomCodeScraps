public class BinarySearch2DArray {
    // This is basically a binary search but inside a matrix
    static boolean searchIn2DMatrix(int[][] mat, int target) {

        // a binary search for matrix like array
        int lowPos = 0, highPos = mat.length - 1;
        while (lowPos <= highPos) {
            // find mid for the row array
            int midPos = lowPos + (highPos - lowPos) / 2;

            // System.out.println("-------");
            // System.out.println("lowPos : " + lowPos);
            // System.out.println("midPos : " + midPos);
            // System.out.println("highPos : " + highPos);

            // check if the matrix is the right one
            if (mat[midPos][0] <= target && mat[midPos][mat[midPos].length - 1] >= target) {
                // this is the row we're looking for
                // do binary search again
                int lowEl = 0, highEl = mat[midPos].length - 1;

                while (lowEl <= highEl) {
                    int midEl = lowEl + (highEl - lowEl) / 2;

                    // System.out.println("++++++++");
                    // System.out.println("lowEl : " + lowEl);
                    // System.out.println("midEl : " + midEl);
                    // System.out.println("highEl : " + highEl);

                    if (mat[midPos][midEl] == target) {
                        return true;
                    } else if (mat[midPos][midEl] > target) { // move left
                        highEl = midEl - 1;
                    } else {
                        lowEl = midEl + 1;
                    }
                }

                return false; // row is found, but element is not found

            } else if (mat[midPos][0] > target) { // move left
                highPos = midPos - 1;
            } else { // move right
                lowPos = midPos + 1;
            }
        }

        return false;
    }
}
