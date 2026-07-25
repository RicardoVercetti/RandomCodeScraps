package org.example.slidingwindow;

import java.util.HashMap;
import java.util.Map.Entry;
import java.util.ArrayList;

public class LongestRepeatingCharacterReplacement {
    // Longest Repeating Character Replacement
    // You are given a string s consisting of only uppercase English characters and an integer k. You can choose up to k characters of the string and replace them with any other uppercase English character.
    // After performing at most k replacements, return the length of the longest substring which contains only one distinct character.
    // Example 1:
    // Input: s = "XYYX", k = 2
    // Output: 4
    // Explanation: Either replace the 'X's with 'Y's, or replace the 'Y's with 'X's.

    // Example 2:
    // Input: s = "AAABABB", k = 1
    // Output: 5

    // Constraints:
    //
    //    1 <= s.length <= 1000
    //    0 <= k <= s.length

    // Time and Space complexity recommendations
    // ?

    public static void main(String[] args) {
        System.out.println("here goes nothing...");
        String inputStr = "AAABABB";
        int k = 1;
        int output = nahIdBruteForceIt(inputStr, k);

        System.out.println("output: " + output);
    }

    public static int nahIdBruteForceIt(String inputStr, int k) {
        // breakdown:
        // 1. find the longest substrings with interruptions, and their interruptions in numbers
        // 2. return the number of which the k replacements is the maximum out of them all.

        HashMap<Character, String> uniqueCharArrays = new HashMap<Character, String>();
        for (char oneItem: inputStr.toCharArray()) {
            if (uniqueCharArrays.containsKey(oneItem)) {
                continue;
            }
            uniqueCharArrays.put(oneItem, getPositionalFrequencyString(inputStr, oneItem));
        }

        int longestCount = 0;
        for(Entry<Character, String> item: uniqueCharArrays.entrySet()) {
            int currentResult = longestSequenceWithKMostGap(item.getValue(), k);
//            System.out.println("char: " + item.getKey() + ", freqStr: " + item.getValue() + ", result: " + currentResult);
            if (currentResult > longestCount) longestCount = currentResult;
        }


        return longestCount;
    }

    // helper DTO
    static class LengthAndGap {
        Integer length;
        Integer gap;

        LengthAndGap(int length, int gap) {
            this.length = length;
            this.gap = gap;
        }

        @Override
        public String toString() {
            return "ln: " + this.length + ", gap: " + this.gap;
        }
    }

    public static int longestSequenceWithKMostGap(String input, int k) {
        ArrayList<LengthAndGap> longestSubString = new ArrayList<LengthAndGap>();
        for (int i=0; i<input.length(); i++) {
            String item = input.substring(i, i+1);
            if (item.equals("0")) {
                // hafta put gaps in all others
                for(LengthAndGap lengthAndGap: longestSubString) {
                    if (lengthAndGap.length > 0 && lengthAndGap.gap < k) {
                        lengthAndGap.gap += 1;
                        lengthAndGap.length += 1;
                    }
//                    else {
//                        lengthAndGap.gap += 1;
//                    }
                }

                // finally add this one's gap entry
                longestSubString.add(new LengthAndGap(0, 0));
            }

            if (item.equals("1")) {
                // hafta increase the length on every item where gap < k
                for (LengthAndGap lengthAndGap: longestSubString) {
                    if (lengthAndGap.length > 0 && lengthAndGap.gap <= k) lengthAndGap.length += 1;
                }

                longestSubString.add(new LengthAndGap(1, 0));
            }
        }

//        System.out.println("--------".repeat(6));
//        for (LengthAndGap value: longestSubString) {
//            System.out.println(value.toString());
//        }
//        System.out.println("--------".repeat(6));


        int longest = 0;
        for (LengthAndGap item: longestSubString) {
            if (item.length > longest) longest = item.length;
        }
        return longest;
    }

    public static String getPositionalFrequencyString(String input, char character) {
        StringBuilder sb = new StringBuilder();
        for (char item: input.toCharArray()) {
            if (item == character) {
                sb.append("1");
            } else {
                sb.append("0");
            }
        }
        return sb.toString();
    }
}
