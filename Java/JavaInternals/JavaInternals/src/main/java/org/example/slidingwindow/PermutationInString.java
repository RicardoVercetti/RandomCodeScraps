package org.example.slidingwindow;

import java.util.HashSet;
import java.util.Set;
import java.util.Stack;

public class PermutationInString {
    // Problem statement:
    // You are given two strings s1 and s2.
    // Return true if s2 contains a permutation of s1, or false otherwise. That means if a permutation of s1 exists as a substring of s2, then return true.
    // Both strings only contain lowercase letters.

    // Example 1:
    // Input: s1 = "abc", s2 = "lecabee"
    // Output: true
    // Explanation: The substring "cab" is a permutation of "abc" and is present in "lecabee".

    // Example 2:
    // Input: s1 = "abc", s2 = "lecaabee"
    // Output: false

    // Constraints:
    //    1 <= s1.length, s2.length <= 1000

    // Time and space complexity recommendations:
    // ?

    // mine:
    // time = O(n)
    // space = O(m)

    public static void main(String[] args) {
        System.out.println("here goes nothing...");

        String s1 = "abc";
        String s2 = "leccabee";
        boolean response = nahIdbruteForceIt(s1, s2);
        System.out.println("output: " + response);

    }

    public static boolean nahIdbruteForceIt(String s1, String s2) {     // but this has a problem, when the first item has to be ignored, this will fail

        Stack<String> initialized = new Stack<String>();
        for (char item: s1.toCharArray()) {
            initialized.add(String.valueOf(item));
        }

        Stack<String> counted = new Stack<String>();
        counted.addAll(initialized);

        for (int i=0; i<s2.length(); i++) {
            String item = s2.substring(i, i + 1);
            if (counted.contains(item)) {
                counted.remove(item);
            } else {
                counted.clear();
                counted.addAll(initialized);
            }
            if (counted.isEmpty()) return true;
        }
        return false;
    }


}
