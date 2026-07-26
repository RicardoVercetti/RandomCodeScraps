package org.example.StackProblems;

import java.util.Stack;

public class EvaluateReversePolishNotation {
    // Problem statement:
    // You are given an array of strings tokens that represents a valid arithmetic expression in Reverse Polish Notation.
    //
    // Return the integer that represents the evaluation of the expression.
    //
    //    The operands may be integers or the results of other operations.
    //    The operators include '+', '-', '*', and '/'.
    //    Assume that division between integers always truncates toward zero.
    //
    // Example 1:
    // Input: tokens = ["1","2","+","3","*","4","-"]
    // Output: 5
    // Explanation: ((1 + 2) * 3) - 4 = 5

    // Constraints:
    //    1 <= tokens.length <= 1000.
    //    tokens[i] is "+", "-", "*", or "/", or a string representing an integer in the range [-200, 200].

    public static int nahIdBruteForceIt(String[] inputArray) {
        int pos = 0;
        Stack<Integer> numberStack = new Stack<Integer>();
        while (pos<inputArray.length) {
            String thisItem = inputArray[pos];

            if (thisItem.equals("+") || thisItem.equals("-") || thisItem.equals("/") || thisItem.equals("*")) {
                // do the operation with the last to elements in the stack
                Integer b = numberStack.pop();
                Integer a = numberStack.pop();
                Integer c = null;

                if (thisItem.equals("+")) {
                    c = a + b;
                } else if (thisItem.equals("-")) {
                    c = a - b;
                } else if (thisItem.equals("/")) {
                    c = a / b;
                } else {
                    c = a * b;
                }

                // push the result back to the stack
                numberStack.add(c);

            } else {
                // this is a number, have to push it to the number stack
                numberStack.push(Integer.valueOf(thisItem));
            }


            pos += 1;
        }
        return numberStack.getFirst();          // if there are more than one element here, it's an invalid expression
    }

    public static void main(String[] args) {
        System.out.println("here goes nothing...");

        String[] inputTokens = {"1","2","+","3","*","4","-", "4"};
        int output = nahIdBruteForceIt(inputTokens);

        System.out.println("output: " + output);
    }
}
