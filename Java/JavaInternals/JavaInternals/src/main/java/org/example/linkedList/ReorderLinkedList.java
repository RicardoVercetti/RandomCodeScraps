package org.example.linkedList;

import java.util.ArrayList;
import java.util.List;

public class ReorderLinkedList {
    // Problem Statement:
    // You are given a head of a singly linked list. The list can be represented as:
    // The positions of a linked list fo length = 7 for example, can be initially represented as:
    // [0, 1, 2, 3, 4, 5, 6]
    // Reorder the nodes of the linked list to be in the following order: [0, 6, 1, 5, 2, 4, 3]
    // Notice that in the general case for a list of length = n, the nodes are reordered in the following order:
    // [0, n-1, 1, n-2, 2, n-3, ...]
    // You may not modify the values in the list's nodes, but instead you must reorder the nodes themselves.
    // eg1:
    // Input: head = [2, 4, 6, 8]
    // Output: [2, 8, 4, 6]
    // eg2:
    // Input: head = [2, 4, 6, 8, 10]
    // Output: [2, 10, 4, 8, 6]

    // Constraints:
    // 1 <= length of the list <= 1000
    // 1 <= node.val = <= 1000

    public static void main(String[] args) {
        System.out.println("here goes nothing...");

        ListNode a = new ListNode(2);
        ListNode b = new ListNode(4);
        ListNode c = new ListNode(6);
        ListNode d = new ListNode(8);
        ListNode e = new ListNode(5);

        a.next = b;
        b.next = c;
        c.next = d;
        d.next = e;

        var out = theSolution(a);
        System.out.println("out: " + out);
    }

    public static ArrayList<Integer> theSolution(ListNode head) {
        // convert the nodes into a list
        // split the list in half, and go one at first and one at the last

        ArrayList<Integer> items = toList(head);
        ArrayList<Integer> returnList = new ArrayList<>();
        int last = items.size() - 1;
        boolean hasEven = last % 2 != 0;
        int mid = hasEven ? last/2 + 1 : last/2;

        for (int i=0; i<mid; i++) {
            returnList.add(items.get(i));
            returnList.add(items.get(last - i));
        }

        if (!hasEven) returnList.add(items.get(mid));

        return returnList;
    }

    public static ArrayList<Integer> toList(ListNode nodes) {
        ArrayList<Integer> returnList = new ArrayList<>();
        ListNode node = nodes;
        while(node != null) {
            returnList.add(node.val);
            node = node.next;
        }
        return returnList;
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
