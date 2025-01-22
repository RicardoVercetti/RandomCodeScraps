package Java.Mastery.Algorithms;

// import java.util.Arrays;

import java.util.Arrays;


public class TwoSum {

    // Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

    static int[] twoSum(int[] arr, int target) {
        int[] returnArr = new int[2];
        for (int i=0; i<arr.length; i++) {
            for (int j=0; j<arr.length; j++) {
                if(i==j) continue;
                if((arr[i]+arr[j] == target)) {
                    returnArr[0] = i;
                    returnArr[1] = j;
                    return returnArr;
                }
            }
        }
        System.out.println("No results found");
        return null;
    }

    public static void main(String[] args) {
        int[] arr = {1, 2, 3, 4, 5};
        int target = 9;
        System.out.println(Arrays.toString(twoSum(arr, target)));
    }
}
