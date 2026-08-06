package org.example.linkedList;

import java.util.ArrayList;

public class MergeTwoSortedLinkedLists {
    // Problem statement:
    // You are given the heads of two sorted linked lists list1 and list2.
    // Merge the two lists into one sorted linked list and return the head of the new sorted linked list.
    // The new list should be made up of nodes from list1 and list2.

    //Example 1:
    // Input: list1 = [1,2,4], list2 = [1,3,5]
    // Output: [1,1,2,3,4,5]

    // Example 2:
    // Input: list1 = [], list2 = [1,2]
    // Output: [1,2]

    // Example 3:
    // Input: list1 = [], list2 = []
    // Output: []

    // Constraints:
    //    0 <= The length of each list <= 100.
    //    -100 <= Node.val <= 100

    public static void main(String[] args) {
        System.out.println("here goes nothing..");
        ListNode node1 = new ListNode(1);
        ListNode node2 = new ListNode(2);
        ListNode node3 = new ListNode(3);
        ListNode node4 = new ListNode(4);

        node1.next = node2;
        node2.next = node3;
        node3.next = node4;

        ListNode node5 = new ListNode(5);
        ListNode node6 = new ListNode(6);
        ListNode node7 = new ListNode(7);

        node5.next = node6;
        node6.next = node7;

        System.out.println("node1: " + display(node1));
        System.out.println("node2: " + display(node5));

        ListNode output = hereGoesNothing(node1, node5);

        System.out.println("output: " + display(output));
    }

    public static ArrayList<Integer> display(ListNode input) {
        ArrayList<Integer> arr = new ArrayList<>();
        ListNode ref = input;
        do {
            arr.add(ref.val);
            ref = ref.next;
        } while (ref != null);

//        System.out.println("list: " + arr);
        return arr;
    }

    public static ListNode hereGoesNothing(ListNode node1, ListNode node2) {
        ListNode output = null;
        ListNode outFirst = null;

        ListNode firstNode = node1;
        ListNode secondNode = node2;
        boolean isBothNotExhausted = true;

        int n = 0;

        while (isBothNotExhausted) {
            if (firstNode == null && secondNode != null) {
                output.next = new ListNode(secondNode.val);
                secondNode = secondNode.next;
                output = output.next;
            } else if (firstNode != null && secondNode == null) {
                output.next = new ListNode(firstNode.val);
                firstNode = firstNode.next;
                output = output.next;
            } else if (firstNode == null && secondNode == null) {
                isBothNotExhausted = false;
            } else {
                if (firstNode.val >= secondNode.val) {
                    if (output == null) {
                        output = new ListNode(secondNode.val);
                        secondNode = secondNode.next;
                        outFirst = output;
                    } else {
                        output.next = new ListNode(secondNode.val);
                        secondNode = secondNode.next;
                        output = output.next;
                    }
                } else {
                    if (output == null) {
                        output = new ListNode(firstNode.val);
                        firstNode = firstNode.next;
                        outFirst = output;
                    } else {
                        output.next = new ListNode(firstNode.val);
                        firstNode = firstNode.next;
                        output = output.next;
                    }
                }
            }
        }
        return outFirst;
    }

    public static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode next) {
            this.val = val;
            this.next = next;
        }
    }
}
