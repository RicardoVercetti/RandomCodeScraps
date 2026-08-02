package org.example.binarySearch;

public class SearchInRotatedSortedArray {
    // Problem Statement:
    // You are given an array of length n which was originally sorted in ascending order. It has now been rotated between 1 and n times. For example, the array nums = [1,2,3,4,5,6] might become:
    //
    //    [3,4,5,6,1,2] if it was rotated 4 times.
    //    [1,2,3,4,5,6] if it was rotated 6 times.
    //
    // Given the rotated sorted array nums and an integer target, return the index of target within nums, or -1 if it is not present.
    // You may assume all elements in the sorted rotated array nums are unique,
    // A solution that runs in O(n) time is trivial, can you write an algorithm that runs in O(log n) time?

    // Example 1:
    // Input: nums = [3,4,5,6,1,2], target = 1
    // Output: 4

    // Example 2:
    // Input: nums = [3,5,6,0,1,2], target = 4
    // Output: -1

    // Constraints:
    //
    //    1 <= nums.length <= 1000
    //    -1000 <= nums[i] <= 1000
    //    -1000 <= target <= 1000
    //    All values of nums are unique.
    //    nums is an ascending array that is possibly rotated.

    public static void main(String[] args) throws InterruptedException {
        System.out.println("here goes nothing...");
        int[] inputArray = {16, 17, 18, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 };
        int result = letsSeeWhatIcanDo(inputArray, 18);

        System.out.println("output: " + result);
    }


    public static int letsSeeWhatIcanDo(int[] inputArray, int target) throws InterruptedException {
        // start from leftmost and mark middle
        // if leftmost and the middle are ascending,
        //      - and the target is inbetween - we go left from middle.
        //              set leftmost = unchanged
        //                  rightmost = middle
        //                  middle = set by repeating the split
        //      - else - we go right from middle
        //              set leftmost = middle
        //                  rightmost = unchanged
        //                  middle = set by repeating the split
        // else if not ascending(meaning rhs must be ascending), we have to check
        //      - if middle < target && rightmost > target
        //              set leftmost = middle
        //                  rightmost = unchanged
        //                  middle = set by repeating the split
        //      - else <the cutoff is here>
        //              set leftmost = unchanged
        //                  rightmost = middle
        //                  middle = set by repeating the split

        int leftMost = 0;
        int rightMost = inputArray.length -1;
        int middle = (rightMost - leftMost)/2 + leftMost;

        while (leftMost < rightMost) {
//            System.out.println("(l, m, r)=" + "(" + leftMost + ", " + middle + ", " + rightMost + ")");
//            Thread.sleep(3000);
            // hafta check leftmost, rightmost and the middle for target
            if(inputArray[leftMost] == target) return leftMost;
            if(inputArray[rightMost] == target) return rightMost;
            if(inputArray[middle] == target) return middle;

            if (isAscending(inputArray, leftMost, middle)) {
                if (inputArray[leftMost] < target && inputArray[middle] > target) {
//                    System.out.println("first IF, IF");
                    rightMost = middle;
                    middle = (rightMost - leftMost)/2 + leftMost;
                } else {
//                    System.out.println("first IF, ELSE");
                    leftMost = middle;
                    middle = (rightMost - leftMost)/2 + leftMost;
                }
            } else {
                if (inputArray[middle] > target && inputArray[rightMost] < target) {
//                    System.out.println("second ELSE, IF");
                    leftMost = middle;
                    middle = (rightMost - leftMost)/2 + leftMost;
                } else {        // the cutoff is here
//                    System.out.println("second ELSE, ELSE");
                    rightMost = middle;
                    middle = (rightMost - leftMost)/2 + leftMost;
                }
            }
        }
        return -1;
    }

    public static boolean isAscending(int[] inputArray, int leftPos, int rightPos) {
        return inputArray[leftPos] < inputArray[rightPos];
    }
}
