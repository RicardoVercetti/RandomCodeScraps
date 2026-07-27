package org.example.StackProblems;

import java.util.ArrayList;

public class LargestRectangleInHistogram {
    // Problem statement:
    // You are given an array of integers heights where heights[i] represents the height of a bar. The width of each bar is 1.
    // Return the area of the largest rectangle that can be formed among the bars.
    // Note: This chart is known as a histogram.

    // Example 1:
    // Input: heights = [7,1,7,2,2,4]
    // Output: 8

    // Example 2:
    // Input: heights = [1,3,7]
    // Output: 7

    // Constraints:
    //    1 <= heights.length <= 1000.
    //    0 <= heights[i] <= 1000

    // Recommended Complexity:
    // time: O(n)
    // space: O(n)

    public static void main(String[] args) {
        System.out.println("here goes nothing...");
        int[] inputArray = {7,1,7,2,2,4};
        int result = nahIdBruteForceIt(inputArray);

        System.out.println("output: " + result);

    }

    public static int nahIdBruteForceIt(int[] inputArray) {
        // Approach:
        // go by each item, for each item in the row, there is a cap length - maximum length to consider for the area calculation
        // when the next item has lesser length than the former, the cap length shrinks, if next one higher, it continues
        // when the shrinking happens, we'll have to calculate the area right there before the shrinking.
        // any calculated point that will have

        // Draggable logic: for each item,
        //  i. item[pos] > item[pos+1] = cap and drag by item[pos+1], unless its 1(cutoff here).
        //  ii. item[pos] <= item[pos+1] = draggable by item[pos], keep dragging

        int largestArea = 0;
        ArrayList<Draggable> draggables = new ArrayList<>();

        for (int i=0; i<inputArray.length; i++) {
            // check draggables for others
            for (int j=0; j<draggables.size(); j++) {

                // cutoff dragging
                if (inputArray[i] == 1) {
                    int area = draggables.get(j).currentCap * (draggables.get(j).currentPos + 1 - draggables.get(j).pos);
                    if (area > largestArea) largestArea = area;
                    draggables.remove(j);       // remove this dragging
                    j--;
                    continue;
                }

                // calculate cutoff and update draggable
                if (draggables.get(j).currentCap > inputArray[i] && inputArray[i] != 1) {
                    int area = draggables.get(j).currentCap * (draggables.get(j).currentPos + 1 - draggables.get(j).pos);
                    if (area > largestArea) largestArea = area;
                    draggables.get(j).currentCap = inputArray[i];
                }

                draggables.get(j).currentPos = i;           // don't have to track current pos though
            }

            // set draggable for this
            if (inputArray[i] != 1) {
                if (inputArray[i] > largestArea) largestArea = inputArray[i];
                draggables.add(new Draggable(i, inputArray[i]));
            }
        }

//        System.out.println("allDraggables: " + draggables);

        for (Draggable draggable: draggables) {
            int areaCalc = draggable.currentCap * (draggable.currentPos + 1 - draggable.pos);
            if (areaCalc > largestArea) {
                largestArea = areaCalc;
            }
        }

        // calculate the minimum cap(1 * n) at last
        if (inputArray.length > largestArea) largestArea = inputArray.length;

        return largestArea;
    }

    static class Draggable {
        Integer pos;
        Boolean is_draggable;
        Integer currentCap;
        Integer currentPos;

        public Draggable(int pos, int currentCap) {
            this.pos = pos;
            this.currentCap = currentCap;
            this.is_draggable = true;
            this.currentPos = pos;
        }

        @Override
        public String toString() {
            return "{pos=" + pos + ", currentCap: " + currentCap + ", currentPos: " + currentPos + "}";
        }
    }
}
