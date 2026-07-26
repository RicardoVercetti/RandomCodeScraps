package org.example.StackProblems;

import java.util.Arrays;

public class DailyTemperatures {
    // Problem statement:
    // You are given an array of integers temperatures where temperatures[i] represents the daily temperatures on the ith day.
    // Return an array result where result[i] is the number of days after the ith day before a warmer temperature appears on a future day. If there is no day in the future where a warmer temperature will appear for the ith day, set result[i] to 0 instead.

    // Example 1:
    // Input: temperatures = [30,38,30,36,35,40,28]
    // Output: [1,4,1,2,1,0,0]

    // Example 2:
    // Input: temperatures = [22,21,20]
    // Output: [0,0,0]

    // Constraints:
    //
    //    1 <= temperatures.length <= 1000.
    //    1 <= temperatures[i] <= 100

    // Complexity
    // ?

    public static int[] nahIdBruteForceIt(int[] inputArray) {
        boolean[] foundLower = new boolean[inputArray.length];
        int[] nDays = new int[inputArray.length];

        for (int i=0; i<inputArray.length; i++) {
            // for each item, check the items before where the foundLower is not true
            for (int j=0; j<i; j++) {
                if (!foundLower[j] && inputArray[i] > inputArray[j]) {
                    nDays[j] = i-j;
                    foundLower[j] = true;
                }
            }
        }

        return nDays;
    }

    public static void main(String[] args) {
        System.out.println("here goes nothing... really...");

        int[] temperatures = {22,21,20};
        int[] output = nahIdBruteForceIt(temperatures);

        System.out.println("output: " + Arrays.toString(output));
    }
}
