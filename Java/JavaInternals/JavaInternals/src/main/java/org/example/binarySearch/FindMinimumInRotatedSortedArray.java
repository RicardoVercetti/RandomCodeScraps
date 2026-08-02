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
        // start from comparing the leftmost
        // first check if current is the smallest(arr[p] < arr[p-1]), and also check if current + 1 is the smallest.
        // cut the remaining in half by getting the middle, check if it's ascending from the last position.
        // if ascending, check the current + 1 is the smallest.
        // if not, cap that position as the right most and move left and cut middle.
        // keep cutting and checking until the left most and the right most becomes 1 digit apart.
        // then check the same(arr[p] < arr[p-1])

        int leftMost = 0;
        int rightMost = inputArray.length-1;
        int middle = (rightMost - leftMost)/2 + leftMost;
        int lastMin = inputArray[0];
        while (leftMost < rightMost) {
//            System.out.println("(l, m, r, lm)=(" + leftMost + ", " + middle + ", " + rightMost + ", " + lastMin + ")");
            // check leftmost and right most
            if (isLowest(inputArray, leftMost)) return inputArray[leftMost];
            if (isLowest(inputArray, rightMost)) return inputArray[rightMost];

            if (isLowest(inputArray, middle)) {
//                System.out.println("isLowestIs true for pos: " + middle);
                return inputArray[middle];
            } else {
                // keep cutting based on last min value
                if (lastMin < inputArray[middle]) {     // still ascending, gotta go right
                    lastMin = inputArray[middle];
                    leftMost = middle;
                    middle = (rightMost - leftMost)/2 + leftMost;

                } else if (lastMin > inputArray[middle]) {      // its descending, the cutoff is either current pos or on the left side
                    rightMost = middle;
                    middle = (rightMost - leftMost)/2 + leftMost;
                }
            }
        }

        return -1;
    }

    public static boolean isLowest(int[] inputArray, int pos) {
        if (pos - 1 < 0) { // its comparison of first and last
            return inputArray[0] < inputArray[inputArray.length-1];
        }
        return inputArray[pos-1] > inputArray[pos];
    }

}
