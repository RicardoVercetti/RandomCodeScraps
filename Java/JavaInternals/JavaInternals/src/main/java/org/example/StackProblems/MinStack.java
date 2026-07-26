package org.example.StackProblems;

import java.util.Arrays;

public class MinStack {
    // Problem Statement:
    // Design a stack class that supports the push, pop, top, and getMin operations.
    //
    //    MinStack() initializes the stack object.
    //    void push(int val) pushes the element val onto the stack.
    //    void pop() removes the element on the top of the stack.
    //    int top() gets the top element of the stack.
    //    int getMin() retrieves the minimum element in the stack.
    //
    // Each function should run in O(1)O(1) time.
    //
    // Example 1:
    //
    // Input: ["MinStack", "push", 1, "push", 2, "push", 0, "getMin", "pop", "top", "getMin"]
    //
    // Output: [null,null,null,null,0,null,2,1]
    //
    // Explanation:
    // MinStack minStack = new MinStack();
    // minStack.push(1);
    // minStack.push(2);
    // minStack.push(0);
    // minStack.getMin(); // return 0
    // minStack.pop();
    // minStack.top();    // return 2
    // minStack.getMin(); // return 1

    // Constraints:
    //    -2^31 <= val <= 2^31 - 1.
    //    pop, top and getMin will always be called on non-empty stacks.

    // my time complexity:
    // all are O(1)
    // except: pop(), when a new min have to be found, its O(n)

    Integer size;
    Integer maxSize;
    int [] arrayOfValues;
    int minElementPos;

    MinStack(int maxSize) {
        this.maxSize = maxSize;
        this.arrayOfValues = new int[maxSize];
        this.size = 0;
        this.minElementPos = -1;
    }

    void push(int val) {
        this.arrayOfValues[this.size] = val;
        if (this.minElementPos == -1 || this.arrayOfValues[this.minElementPos] > val) this.minElementPos = this.size;
        this.size += 1;
    }

    void pop() {
        if (this.size == 0) return;
        this.arrayOfValues[this.size-1] = 0;
        this.size -= 1;

        // refind min element position:
        if (this.minElementPos == this.size) {
            this.findNextMinElement();
        }
    }

    void findNextMinElement() {
        int minElement = Integer.MAX_VALUE;
        for (int i=0; i<this.size; i++) {
            if (this.arrayOfValues[i] < minElement) {
                this.minElementPos = i;
                minElement = this.arrayOfValues[i];
            }
        }
    }

    int top() {
        if (this.size == 0) return 0;
        return this.arrayOfValues[this.size - 1];
    }

    int getMin() {
        if (this.minElementPos == -1) return 0;
        return this.arrayOfValues[minElementPos];
    }

    public static void main(String[] args) {
        System.out.println("here goes nothing...");

        // input: ["MinStack", "push", 1, "push", 2, "push", 0, "getMin", "pop", "top", "getMin"]
        // output:[null,        null,     null,       null,      0,        null,  2,     1      ]
        MinStack minStack = new MinStack(40);
        minStack.push(1);
        minStack.push(2);
        minStack.push(0);
        System.out.println("getmin: " + minStack.getMin());
        minStack.pop();
        System.out.println("top: " + minStack.top());
        System.out.println("getMin: " + minStack.getMin());
    }
}
