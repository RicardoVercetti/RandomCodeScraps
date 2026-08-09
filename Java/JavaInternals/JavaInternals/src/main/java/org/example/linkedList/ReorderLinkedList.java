package org.example.linkedList;

import java.util.ArrayList;

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
        ListNode e = new ListNode(10);
//        ListNode f = new ListNode(6);
//        ListNode g = new ListNode(7);

        a.next = b;
        b.next = c;
        c.next = d;
        d.next = e;
//        e.next = f;
//        f.next = g;

        System.out.println("input: " + displayList(a));
        theSolution(a);
        System.out.println("out: " + displayList(a));
    }

    public static void theSolution(ListNode head) {
        // no need to convert to list, have a fast and slow pointer so that we can split the node into two.
        // break the connection of the middle one, reverse the second half by rearranging the connections
        // merge both by taking one from each.

        ListNode slow = head;
        ListNode fast = head.next;

        while (fast != null && fast.next != null) {
            slow = slow.next;
            fast = fast.next.next;
        }

        // now, reverse the second half
        ListNode second = slow.next;
        slow.next = null;
        ListNode prev = null;
        while (second != null) {
            ListNode temp = second.next;
            second.next = prev;
            prev = second;
            second = temp;
        }

        // go by each and keep adding into the node
        ListNode firstHead = head;
        ListNode secondHead = prev;
        while (secondHead != null) {
            ListNode tempFirst = firstHead.next;
            ListNode tempSecond = secondHead.next;
            firstHead.next = secondHead;
            secondHead.next = tempFirst;
            firstHead = tempFirst;
            secondHead = tempSecond;
        }
    }

    public static ArrayList<Integer> displayList(ListNode ln) {
        ArrayList<Integer> arr = new ArrayList<>();

        ListNode head = ln;
        while (head != null) {
            arr.add(head.val);
            head = head.next;
        }
        return arr;
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
