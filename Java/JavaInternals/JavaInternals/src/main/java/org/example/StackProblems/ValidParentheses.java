package org.example.StackProblems;

import java.util.Stack;

public class ValidParentheses {
    // Problem statement:
    // You are given a string s consisting of the following characters: '(', ')', '{', '}', '[' and ']'.
    //
    // The input string s is valid if and only if:
    //
    //    Every open bracket is closed by the same type of close bracket.
    //    Open brackets are closed in the correct order.
    //    Every close bracket has a corresponding open bracket of the same type.
    //
    // Return true if s is a valid string, and false otherwise.

    //Example 1:
    // Input: s = "[]"
    // Output: true

    // Example 2:
    // Input: s = "([{}])"
    // Output: true

    // Example 3:
    // Input: s = "[(])"
    // Output: false
    // Explanation: The brackets are not closed in the correct order.

    // Constraints:
    //    1 <= s.length <= 1000

    public static void main(String[] args) {
        System.out.println("here goes nothing....");
        String inputString = "[(])";
        System.out.println("input: " + inputString);
        boolean output = iAlreadyKnowTheSolution(inputString);
        System.out.println("output: " + output);
    }

    public static boolean iAlreadyKnowTheSolution(String inputString) {
        Stack<Character> paranthesisStack = new Stack<>();
        for (char paranthesis: inputString.toCharArray()) {
            if ('{' == paranthesis) {
                paranthesisStack.add('}');
            } else if ('[' == paranthesis) {
                paranthesisStack.add(']');
            } else if ('(' == paranthesis) {
                paranthesisStack.add(')');
            } else if ('}' == paranthesis || ']' == paranthesis || ')' == paranthesis) {
                if (paranthesis == paranthesisStack.peek()) {
                    paranthesisStack.pop();
                }
            }
        }
        return paranthesisStack.isEmpty();
    }
}
