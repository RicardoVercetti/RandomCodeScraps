package org.example.linkedList;

import java.util.ArrayList;

public class ReverseLinkedList {
    // Problem statement:
    // Given the beginning of a singly linked list head, reverse the list, and return the new beginning of the list.

    // Example 1:
    // Input: head = [0,1,2,3]
    // Output: [3,2,1,0]

    // Example 2:
    // Input: head = []
    // Output: []

    // Constraints:
    //
    //    0 <= The length of the list <= 1000.
    //    -1000 <= Node.val <= 1000

    public static void main(String[] args) {
        System.out.println("here goes nothing..");
        ListNode node1 = new ListNode(1);
        ListNode node2 = new ListNode(2);
        ListNode node3 = new ListNode(3);
        ListNode node4 = new ListNode(4);
        node1.next = node2;
        node2.next = node3;
        node3.next = node4;

        listAllNodes(node1);


        ListNode result = hereGoesNothing(node1);

        listAllNodes(result);

    }

    public static ListNode hereGoesNothing(ListNode input) {
        ListNode source = input;
        ListNode target = null;
        boolean isNotEnded = true;

        int n = 0;
        while (isNotEnded) {
            n++;
            if (source != null) {
                if (target != null) {
                    ListNode node = new ListNode(source.val);
                    node.next = target;
                    source = source.next;
                    target = node;

                } else {
                    target = new ListNode(source.val);
                    source = source.next;
                }
            } else {
                isNotEnded = false;
            }
        }

        return target;
    }

    public static void listAllNodes(ListNode nodes) {
        ListNode node = nodes;
        ArrayList<Integer> list = new ArrayList<>();
        do {
            list.add(node.val);
            node = node.next;
        } while(node != null);

        System.out.println("list: " + list);
    }

    public static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode node) {
            this.val = val;
            this.next = node;
        }

        @Override
        public String toString() {
            return "v: " + this.val + ", " + this.next;
        }
    }
}
