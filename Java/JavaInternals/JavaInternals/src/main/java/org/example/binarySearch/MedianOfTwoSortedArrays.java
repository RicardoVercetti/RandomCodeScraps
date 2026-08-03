package org.example.binarySearch;

import java.util.ArrayList;
import java.util.Arrays;

public class MedianOfTwoSortedArrays {
    // Problem statement:
    // You are given two integer arrays `nums1` and `nums2` of size m and n respectively, where each is sorted in ascending order.
    // Return the median value among all elements of the two arrays.
    // Your solution should run in O(log(m+n)) time.

    // Example 1:
    // Input: nums1 = [1, 2], nums2 = [3]
    // Output: 2.0
    // Explanation: Among [1, 2, 3] the median is 2.

    // Example 2:
    // Input: nums1 = [1, 3], nums2 = [2, 4]
    // Output: 2.5
    // Explanation: Among [1, 2, 3, 4] the median is (2 + 3) / 2 = 2.5

    // Constraints:
    // nums1.length == m
    // nums2.length == n
    // 0 <= m <= 1000
    // 0 <= n <= 1000
    // 1 <= m + n <= 2000
    // -10^6 <= nums1[i], nums2[i] <= 10^6

    // Recommended time & space complexity:
    // Time: O(log(min(n, m)))
    // space: O(1)
    // where n is size of nums1 and m is the size of nums2

    public static float findMedian(int[] arr1, int[] arr2) {
        ArrayList<Integer> arrayList = new ArrayList<>(arr1.length);
        for (int value: arr1) {
            arrayList.add(value);
        }

        // now, binary insert each number from arr2
        for (int number: arr2) {
            int start = 0;
            int end = arrayList.size() - 1;
            int mid = (end - start)/2 + start;

            while (start <= end) {
//                System.out.println("while start: " + start + ", mid: " + mid + ", end: " + end);
                int thisItem = arrayList.get(mid);
                if (number < thisItem) {
                    end = mid;
                } else if (number > thisItem) {
                    start = mid;
                }

                mid = (end - start)/2 + start;

                if ((start == mid & end - 1 == mid) || (end == mid && mid - 1 == start)) break;
            }
//            System.out.println("num: " + number + ", mid: " + mid);
            arrayList.add(mid, number);
        }
        System.out.println("sorted array: " + arrayList);

        int size = arrayList.size();

        // calculate the median
        if (arrayList.size() % 2 == 0) {
            int mid_first = size / 2 - 1;
            int mid_second = mid_first + 1;

            return ((float)arrayList.get(mid_first) + (float)arrayList.get(mid_second))/2;
        }

        return (float) arrayList.get(size - 1);
    }

    public static void main(String[] args) {
        System.out.println(1/2);
        System.out.println("here goes nothing...");
        int[] nums1 = {1, 2};
        int[] nums2 = {3};

        float median = findMedian(nums1, nums2);

        System.out.println("output: " + median);

    }
}
