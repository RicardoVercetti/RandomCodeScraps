package org.example.slidingwindow;

import java.util.HashMap;
import java.util.Map.Entry;

public class LongestSubStringWithoutRepeatingCharacters {

    // Problem statement
    // Given a string `s`, find the length of the longest substring without duplicate characters
    // A substring is a contiguous sequence of characters with string
    // eg-1:
    // Input: s = "zxyzxyz"
    // Output:  3

    // eg-2:
    // Input: s = "xxxx"
    // Output: 1

    // Constraints:
    // 0 <= s.length <= 100
    // `s` may contain printable ASCII characters

    // Recommended time and space complexity
    // time: O(n)       n = length of the string
    // space: O(m)      m = number of unique characters in string



    // other notes:
    // 1. there are quirks when it comes to accessing and modifying things in HashMap


    public static void main(String[] args) {
        System.out.println("here goes nothing...");
        String s = "zxyzxyz";
        int output = aBetterSolution(s);

        System.out.println("output: " + output);
    }

    public static int aBetterSolution(String inputStr) {
        // step-1: for each character, put in a hash map with the count 1
        // step-2: for each character in the list, increase the count of all characters in map by 1 except for the one that matches with the current character
        // step-3: As of the matched one, take the count from the map and save if it's the largest among the other pushed lengths and reset the count to 1.
        int largestPushedLength = 0;

        HashMap<Character, Integer> countPerChar = new HashMap<Character, Integer>();

        for (char item: inputStr.toCharArray()) {
            // step-2
            for(Entry<Character, Integer> thisSet: countPerChar.entrySet()) {   // other character count increments
                if (thisSet.getKey() != item) {
                    thisSet.setValue(thisSet.getValue() + 1);
                };

            }

            // step-3
            if (countPerChar.containsKey(item)) {
                int lastCountOfChar = countPerChar.get(item);
                if(lastCountOfChar > largestPushedLength) largestPushedLength = lastCountOfChar;

                countPerChar.put(item, 1);
            }

            // step-1
            if (!countPerChar.containsKey(item)) { // first time insert
                countPerChar.put(item, 1);
            }
        }

        // finally, we also add all the remaining counts from the hashMap
        for (Integer count: countPerChar.values()) {
            if(count > largestPushedLength) largestPushedLength = count;
        }

        return largestPushedLength;
    }

    public static int nahIdBruteForceIt(String inputStr) {
        // here the time complexity is O(nlog(n)) and space complexity is O(1)
        int largestSubStringCount = 0;
        for (int i=0; i<inputStr.length(); i++) {
            int subStringCount = 1;
            for (int j=i+1; j<inputStr.length(); j++) {
                if (inputStr.substring(i, i+1).equals(inputStr.substring(j, j+1))) {
                    if (subStringCount > largestSubStringCount) largestSubStringCount = subStringCount;
                    break;
                }
                subStringCount += 1;
            }
        }

        return largestSubStringCount;
    }
}
