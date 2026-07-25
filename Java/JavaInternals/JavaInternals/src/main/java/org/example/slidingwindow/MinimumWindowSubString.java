package org.example.slidingwindow;

import java.util.ArrayList;
import java.util.Stack;

public class MinimumWindowSubString {
    // problem statement:
    // Given two strings s and t, return the shortest substring of s such that every character in t, including duplicates, is present in the substring. If such a substring does not exist, return an empty string "".
    // You may assume that the correct output is always unique.

    // Example 1:
    // Input: s = "OUZODYXAZV", t = "XYZ"
    // Output: "YXAZ"
    // Explanation: "YXAZ" is the shortest substring that includes "X", "Y", and "Z" from string t.

    // Example 2:
    // Input: s = "xyz", t = "xyz"
    // Output: "xyz"

    // Example 3:
    // Input: s = "x", t = "xy"
    // Output: ""

    // Constraints:
    //    1 <= s.length <= 1000
    //    1 <= t.length <= 1000
    //    s and t consist of uppercase and lowercase English letters.

    // Recommended time & space complexity
    //

    public static void main(String[] args) {
        System.out.println("here goes nothing...");
        String s = "x";
        String t = "xy";
        String output = nahIdBruteForceIt(s, t);
        System.out.println("output: " + output);
    }

    static class SubStringElement {
        Integer startPosition;
        Integer endPosition;
        Stack<String> removedStack;
        public SubStringElement(int startPos, Stack<String> initStack) {
            this.removedStack = initStack;
            this.startPosition = startPos;
        }

        @Override
        public String toString() {
            return "startPos: " + this.startPosition + ", endPos: " + this.endPosition + ", removedStack: " + this.removedStack.toString();
         }
    }

    public static String nahIdBruteForceIt(String s, String t) {
        // for each item in t, I have to check if its from s and mark if the start position and also removing the item from the stack.
        // when the stack is empty, have to mark the end position.
        // when the end position is marked, that's a candidate substring, if it's the smallest one by the end, return that, else return ""
        ArrayList<SubStringElement> allSubStrings = new ArrayList<>();
        Stack<String> initStack = new Stack<String>();
        for (char item: t.toCharArray()) {
            initStack.add(String.valueOf(item));
        }

        for (int i=0; i<s.length(); i++) {
            String strInQuestion = s.substring(i, i+1);

            if (t.contains(strInQuestion)) {
                // increment other item in the list
                for (SubStringElement element: allSubStrings) {
                    element.removedStack.remove(strInQuestion);
                    if (element.removedStack.isEmpty()) element.endPosition = i;
                }

                // add this item
                Stack<String> newStack = new Stack<String>();
                newStack.addAll(initStack);
                newStack.remove(strInQuestion);
                allSubStrings.add(new SubStringElement(i, newStack));
            }

        }

//        allSubStrings.forEach(item -> System.out.println(item.toString()));

        // now take the smallest pos
        int smallest = 1001;
        int startpos = 1001;
        int endPos = 1001;
        for (SubStringElement element: allSubStrings) {
            if (element.removedStack.isEmpty()) {
                int diff = element.endPosition - element.startPosition;
                if (diff < smallest) {
                    smallest = diff;
                    startpos = element.startPosition;
                    endPos = element.endPosition;
                }
            }
        }

        if (smallest == 1001) return "";
        return s.substring(startpos, endPos+1);
    }


}
