package org.example.slidingwindow;

import java.util.ArrayList;

public class BestTimeToBuyAndSellStock {
    // Best Time To Buy And Sell Stock
    // You are given an integer array `prices` where `prices[i]` is the price of NeetCoin on the `ith` day.
    // you may choose a single day to buy one NeetCoin and choose different day in the future to sell it.

    // Return the maximum profit you can achieve.
    // You may choose not to make any transactions, in which case the profit would be 0

    // eg-1:
    // Input: `prices = [10, 1, 5, 6, 7, 1]`
    // Output: 6
    // explanation: Buy `prices[1]` and sell `prices[4]`, profit = 7 - 1 = 6

    // eg-2:
    // Input: `prices = [10, 8, 7, 5, 2]`
    // output: 0
    // Explanation: No profitable transactions can be made, thus the max profit in 0

    // Constraints:
    // 1 <= prices.length <= 100
    // 0 <= prices[i] <= 100

    // Recommended time & space
    // time - O(n)
    // space - O(1)

    public static void main(String[] args) {
        System.out.println("here goes nothing...");
        int[] prices = { 10, 1, 5, 6, 7, 1 };
        int maxProfit = aBetterSolution(prices);

        System.out.println("maxProfit is: " + maxProfit);

    }

    public static int aBetterSolution(int[] input) {
        // there are two things involved here
        // 1. finding the leftmost smallest. - go by each and keep updating it.
        // 2. finding the rightmost(or largest after the leftmost's position) largest. - go by each and keep updating it.
        // if the lowest is reset, highest must be after that point, so reset highest
        // if the highest is reset, we can go along
        // by the end of the array, we can simply calculate the difference

        int lowest = 100;
        int highest = 0;
        for (int i=0; i<input.length; i++) {
            if (input[i] < lowest) {
                lowest = input[i];
                highest = 0;
            }
            if (input[i] > highest) highest = input[i];
        }

//        System.out.println("(lowest, highest): ("+ lowest + "," + highest + ")");
        return highest-lowest;
    }

    public static int nahIdBruteForceIt(int[] input) {
        ArrayList<Integer> diffArray = new ArrayList<Integer>();

        for (int i=0; i<input.length; i++) {
            for (int j=i+1; j<input.length; j++) {
                int diff = input[j] - input[i];
                diffArray.add(diff);
            }
        }

        // find largest out of this, and send
        return findLargest(diffArray);
    }

    public static int findLargest(ArrayList<Integer> list) {
        int largest = 0;
        for (Integer i: list) {
            if (i>largest) largest = i;
        }

//        if (largest < 1) largest = 0;         // any -ve value will not be taken cuz we are assigning `largest` to 0
        return largest;
    }
}
