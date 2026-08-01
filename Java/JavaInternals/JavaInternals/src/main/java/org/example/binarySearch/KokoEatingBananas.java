package org.example.binarySearch;

public class KokoEatingBananas {
    // Problem Statement:
    // You are given an integer array piles where piles[i] is the number of bananas in the ith pile. You are also given an integer h, which represents the number of hours you have to eat all the bananas.
    // You may decide your bananas-per-hour eating rate of k. Each hour, you may choose a pile of bananas and eats k bananas from that pile. If the pile has less than k bananas, you may finish eating the pile but you can not eat from another pile in the same hour.
    // Return the minimum integer k such that you can eat all the bananas within h hours.
    //
    // Example 1:
    // Input: piles = [1,4,3,2], h = 9
    // Output: 2
    //Explanation: With an eating rate of 2, you can eat the bananas in 6 hours. With an eating rate of 1, you would need 10 hours to eat all the bananas (which exceeds h=9), thus the minimum eating rate is 2.

    // Example 2:
    // Input: piles = [25,10,23,4], h = 4
    // Output: 25
    // Constraints:

    //    1 <= piles.length <= 1,000
    //    piles.length <= h <= 1,000,000
    //    1 <= piles[i] <= 1,000,000,000

    public static void main(String[] args) {
        System.out.println("here goes nothing...");
        int[] piles = {1, 4, 3, 2};
        int h = 9;

        int output = nahIdBruteForceIt(piles, h);

        System.out.println("output: " + output);
    }

    public static int nahIdBruteForceIt(int[] inputArray, int h) {
        // b = h / len(n) will give a rough idea of minimum number above which the k may occur
        // if ith term is < b, skip
        // if i > b, loop and check h taken for that k is < target
        // keep track of the minimum k until the end

        int candidateK = -1;
        int candidateH = -1;
//        int b = h / inputArray.length;
        for (int i: inputArray) {
//            if (i >= b) {
            // calculate the h for this
            int calculatedH = calculateH(inputArray, i);
            System.out.println("calculatedH: " + calculatedH + ", for: " + i);
            if (candidateK == -1) {
                candidateK = i;
                candidateH = calculatedH;
                System.out.println("setting default candidate: " + i);

            } else if (calculatedH <= h && calculatedH < candidateH && i < candidateK) {
//                lowestH = calculatedH;
                System.out.println("setting candidate: " + i);
                candidateK = i;
                candidateH = calculatedH;
            }
//            }
        }

        return candidateK;
    }

    public static int calculateH(int[] inputArray, int rate) {
        int k = 0;
        int pos = 0;
        int last = inputArray.length;
        while (pos < last) {
            int rem = inputArray[pos] % rate;
            if (rem == 0) {
                k += (inputArray[pos]/rate);
            } else {
                k += (inputArray[pos]/rate) + 1;
            }
            pos += 1;
        }
        return k;
    }
}
