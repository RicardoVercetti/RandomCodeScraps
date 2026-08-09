package org.example.linkedList;

import java.util.List;

public class RemoveNthNodeFromEndOfList {
    // Problem Statement:
    // Given the head of a linked list and integer n, remove the nth node from the end of the list and return its head.

    public static void main(String[] args) {
        System.out.println("here goes nothing...");
        ListNode a = new ListNode(1);
        ListNode b = new ListNode(2);
        ListNode c = new ListNode(3);
        ListNode d = new ListNode(4);

        a.next = b;
        b.next = c;
        c.next = d;

        System.out.println("all: " + a.showAll());

        var newHead = oneSolution(a, 4);

        System.out.println("out: " + (newHead == null ? "null": newHead.showAll()));
    }

    public static ListNode oneSolution(ListNode head, int n) {
        // have two pointers and get the distance apart
        // then unlink the next of the left pointer and make it point to the next of the next item of left
        ListNode dummy = new ListNode(0, head);
        ListNode left = dummy;
        ListNode right = dummy;
        while (right != null && n > 0 ) {
            right = right.next;
            n--;
        }

        if (n != 0 || right == null)  return null;

        // move to the end
        while (right.next != null) {
            right = right.next;
            left = left.next;
        }

        left.next = left.next.next;

        return dummy.next;
    }
}
