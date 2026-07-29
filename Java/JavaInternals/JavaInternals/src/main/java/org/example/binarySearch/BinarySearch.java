package org.example.binarySearch;

public class BinarySearch {
    // Problem statement:
    // You are given an array of distinct integers nums, sorted in ascending order, and an integer target.
    // Implement a function to search for target within nums. If it exists, then return its index, otherwise, return -1.
    // Your solution must run in O(logn)O(logn) time.

    // Example 1:
    // Input: nums = [-1,0,2,4,6,8], target = 4
    // Output: 3

    // Example 2:
    // Input: nums = [-1,0,2,4,6,8], target = 3
    // Output: -1

    // Constraints:
    //    1 <= nums.length <= 10000.
    //    -10000 < nums[i], target < 10000
    //    All the integers in nums are unique.

    public static void main(String[] args) {
        System.out.println("here goes something...");
        int[] nums = {-1,0,2,4,6,8};
        int target = 4;

        int result = someWay(nums, target);
        System.out.println("result: " + result);
    }

    public static int someWay(int[] nums, int target) {
        int startCap = 0;
        int endCap = nums.length;
        int mid = nums.length / 2;

        while (startCap < endCap) {
            if (nums[mid] == target) {
                return mid;
            } else if (nums[mid] > target) { // look left
                endCap = mid - 1;
                mid = ((endCap - startCap) / 2) + startCap;
            } else {            // look right
                startCap = mid + 1;
                mid = ((endCap - startCap) / 2) + startCap;
            }

            if (mid < nums.length && nums[mid] == target) return mid;
        }
        return -1;
    }
}
