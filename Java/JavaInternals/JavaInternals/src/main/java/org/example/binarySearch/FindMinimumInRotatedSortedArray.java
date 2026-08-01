package org.example.binarySearch;

public class FindMinimumInRotatedSortedArray {
    // Problem Statement:
    // You are given an array of length n which was originally sorted in ascending order. It has now been rotated between 1 and n times.
    // For example, the array nums = [1,2,3,4,5,6] might become:
    //    [3,4,5,6,1,2] if it was rotated 4 times.
    //    [1,2,3,4,5,6] if it was rotated 6 times.
    //
    // Notice that rotating the array 4 times moves the last four elements of the array to the beginning. Rotating the array 6 times produces the original array.
    // Assuming all elements in the rotated sorted array nums are unique, return the minimum element of this array.

    // A solution that runs in O(n) time is trivial, can you write an algorithm that runs in O(log n) time?
    // Example 1:
    // Input: nums = [3,4,5,6,1,2]
    // Output: 1

    // Example 2:
    // Input: nums = [4,5,0,1,2,3]
    // Output: 0

    // Example 3:
    // Input: nums = [4,5,6,7]
    // Output: 4

    // Constraints:
    //
    //    1 <= nums.length <= 1000
    //    -1000 <= nums[i] <= 1000

    public static void main(String[] args) {
        System.out.println("here goes nothing...");
        int[] inputArray = {3,4,5,6,1,2};

        int output = mayBeOkaySolution(inputArray);
        System.out.println("output: " + output);
    }

    public static int mayBeOkaySolution(int[] inputArray) {
        // a main loop.
        // start from the left most, and pick the middle
        // the condition is, if the left and right are not in ascending the lowest is in between.
        // if the left and right are in ascending either go left, or the leftmost is the answer.
        // keep shortening the search set until p-1 > p where p is the position

        boolean isNotFound = true;
        int leftMost = 0;
        int rightMost = inputArray.length - 1;
        int middle = inputArray.length / 2;
        while (isNotFound) {
            System.out.println("leftMost: " + leftMost + ", middle: " + middle + ", rightMost: " + rightMost);
            if (rightMost >= inputArray.length) break;
            if (leftMost < 0) break;

            // if the numbers are 1 digit apart, lets check all of them
            if (leftMost<=middle && middle <=rightMost) {
                if (isLowest(inputArray, leftMost)) return inputArray[leftMost];
                if (isLowest(inputArray, middle)) return inputArray[middle];
                if (isLowest(inputArray, rightMost)) return inputArray[rightMost];
            }

            if (isLowest(inputArray, middle)) {
                isNotFound = false;
                return inputArray[middle];
            } else if (isInBetween(inputArray, leftMost, middle)) {
                System.out.println("isInbetween: "+ leftMost + ", " + middle);
                rightMost = middle;
                middle = rightMost - leftMost + rightMost;
            } else {        // move to the right
                leftMost = middle;
                middle = rightMost - 1;
                System.out.println("moving to right: "+ leftMost + ", " + middle);
            }
        }

        return -1;
    }

    public static int getPreviousPos(int[]inputArray, int currentPos) {
        if (currentPos - 1 < 0) return inputArray.length-1;
        return currentPos - 1;
    }

    public static boolean isInBetween(int[] inputArray, int startPos, int endPos) {
        return inputArray[startPos] > inputArray[endPos];
    }

    public static boolean isLowest(int[] inputArray, int pos) {
        if (pos - 1 < 0) { // its comparison of first and last
            return inputArray[0] > inputArray[inputArray.length-1];
        }
        return inputArray[pos-1] > inputArray[pos];
    }

}
