package org.example.slidingwindow;

import java.util.ArrayList;

public class SlidingWindowMaximum {
    // Problem statement:
    // You are given an array of integers `nums` and an integer k. There is a sliding window of size k that starts at the left edge of the array. The window slides one position to the right until it reaches the right edge of the array.
    // Return a list that contains the maximum element in the window at each step.
    // Example 1:
    // Input: nums = [1,2,1,0,4,2,6], k = 3
    // Output: [2,2,4,4,6]

    // Explanation:
    // Window position            Max
    // ---------------           -----
    // [1  2  1] 0  4  2  6        2
    // 1 [2  1  0] 4  2  6        2
    // 1  2 [1  0  4] 2  6        4
    // 1  2  1 [0  4  2] 6        4
    // 1  2  1  0 [4  2  6]       6
    //
    // Constraints:
    //
    //    1 <= nums.length <= 100,000
    //    -10,000 <= nums[i] <= 10,000
    //    1 <= k <= nums.length

    public static void main(String[] args) {
        System.out.println("Here goes nothing...");

        int[] inputArray = {1,2,1,0,4,2,6};
        int k = 3;

        ArrayList<Integer> output = nahIdBruteForceIt(inputArray, k);
        System.out.println("output: " + output);
    }

    public static ArrayList<Integer> nahIdBruteForceIt(int[] inputArray, int k) {
        // when you've hard-found the highest among a row, can keep comparing the next
        // just make sure the last pos is not that highest, if it is, re-hard-find it again

        boolean is_ended = false;
        int start = 0;
        int end = start + k - 1;
        ArrayList<Integer> numbers = new ArrayList<Integer>();

        while (!is_ended) {
               int currentHighestPos = hardFindLargest(inputArray, start, end);
               int currentHighestElement = inputArray[currentHighestPos];
               numbers.add(currentHighestElement);

               if (end >= inputArray.length-1) is_ended = true;
               start += 1;
               end = start + k - 1;
        }

        return numbers;
    }

    public static int hardFindLargest(int[] inputArray, int startPosition, int endPosition) {
        int largestPos =  0;
        int largest = 0;
        for (int i=startPosition; i<=endPosition; i++) {
            if (inputArray[i] > largest) {
                largest = inputArray[i];
                largestPos = i;
            }
        }

        return largestPos;
    }
}
