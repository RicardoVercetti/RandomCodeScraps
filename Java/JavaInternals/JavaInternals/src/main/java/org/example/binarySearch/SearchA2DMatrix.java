package org.example.binarySearch;

public class SearchA2DMatrix {
    // Problem statement:
    // You are given an m x n 2-D integer array matrix and an integer target.
    //    Each row in matrix is sorted in non-decreasing order.
    //    The first integer of every row is greater than the last integer of the previous row.

    // Return true if target exists within matrix or false otherwise.
    // Can you write a solution that runs in O(log(m * n)) time?
    // Example 1:
    // [1,   2,  4,  8]
    // [10, 11, 12, 13]
    // [14, 20, 30, 40]
    // Input: matrix = [[1,2,4,8],[10,11,12,13],[14,20,30,40]], target = 10
    // Output: true

    // Example 2:
    // [1,  2,   4,  8]
    // [10, 11, 12, 13]
    // [14, 20, 30, 40]

    // Input: matrix = [[1,2,4,8],[10,11,12,13],[14,20,30,40]], target = 15
    // Output: false

    public static void main(String[] args) {
        System.out.println("here goes nothing...");

        int[][] matrix = {{1,2,4,8},{10, 11, 12, 13},{14, 20, 30, 40}};
        boolean output = nahIdtrySolvingIt(matrix, 15);
        System.out.println("output: " + output);
    }

    public static boolean nahIdtrySolvingIt(int[][] inputArray, int target) {
        int outerPos = inputArray.length / 2;


        while (outerPos<=inputArray.length && outerPos >= 0) {
            // by each element in
            if (isWithinRange(inputArray[outerPos], target)) {
                // could be first or last
                int innerPos = inputArray[outerPos].length/2;
                int startRange = 0;
                int endRange = inputArray[outerPos].length-1;
                while (startRange <= endRange) {
                    if (inputArray[outerPos][innerPos] == target) {
                        return true;
                    } else if (inputArray[outerPos][innerPos] > target) { // move left
                        endRange = innerPos-1;
                        innerPos = endRange - startRange;
                    } else {        // move right
                        startRange = innerPos+1;
                        innerPos = endRange - startRange;
                    }
                }
            } else if (isGraterThanRange(inputArray[outerPos], target)) {
                // move left
                outerPos -=1;

            } else { // move right
                outerPos  +=1;
            }
        }

        return false;
    }

    public static boolean isWithinRange(int[] rangeInt, int target) {
        return rangeInt[0] <= target && rangeInt[rangeInt.length - 1] >= target;
    }

    public static boolean isLessThanRange(int[] rangeInt, int target) {
        return rangeInt[0] > target;
    }

    public static boolean isGraterThanRange(int[] rangeInt, int target) {
        return rangeInt[rangeInt.length-1] < target;
    }
}
