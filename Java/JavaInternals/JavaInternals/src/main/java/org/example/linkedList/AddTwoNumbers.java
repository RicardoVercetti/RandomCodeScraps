package org.example.linkedList;

import java.util.ArrayList;

public class AddTwoNumbers {
    // Problem statement:
    // You are given two non-empty linked lists, l1 and l2, where each represents a non-negative integer.
    // The digits are stored in reverse order, e.g. the number 321 is represented as 1 -> 2 -> 3 -> in the linked list.
    // Each of the nodes contains a single digit. You may assume the two numbers do not contain any leading zero, except the number 0 itself.
    // Return the sum of the two numbers as a linked list.

    // Example 1:
    // Input: l1 = [1,2,3], l2 = [4,5,6]
    // Output: [5,7,9]
    // Explanation: 321 + 654 = 975.

    // Example 2:
    // Input: l1 = [9], l2 = [9]
    // Output: [8,1]

    // Constraints:
    //    1 <= l1.length, l2.length <= 100.
    //    0 <= Node.val <= 9



    public static void main(String[] args) {
        System.out.println("here goes nothing...");
        // listNode-1
        ListNode la1 = new ListNode(1);
        ListNode la2 = new ListNode(2);
        ListNode la3 = new ListNode(3);
        la1.next = la2;
        la2.next = la3;

        // ListNode-2
        ListNode lb1 = new ListNode(4);
        ListNode lb2 = new ListNode(5);
        ListNode lb3 = new ListNode(6);
        lb1.next = lb2;
        lb2.next = lb3;

        // Sample 2
//        ListNode la1 = new ListNode(9);
//        ListNode lb1 = new ListNode(9);

        var l1full = toArray(la1);
        var l2full = toArray(lb1);
        System.out.println("la1: " + l1full);
        System.out.println("lb1: " + l2full);

        ListNode out = findSum(la1, lb1);
        var outFull = toArray(out);
        System.out.println("out: " + outFull);


    }

    public static ArrayList<String> toArray(ListNode head) {
        ArrayList<String> arr = new ArrayList<>();
        ListNode header = head;
        while (header != null) {
            arr.add("(" + header.val + ")");
            header = header.next;
        }
        return arr;
    }

    public static ListNode findSum(ListNode l1, ListNode l2) {

        ListNode la = l1;
        ListNode lb = l2;
        ListNode head = null;
        ListNode last = null;
        int rem = 0;

        while(true) {
            if (la == null & lb == null & rem == 0) break;

            int a = la != null ? la.val : 0;
            int b = lb != null ? lb.val : 0;

            int c = a + b;

            if (rem != 0) {
                c += rem;
                rem = 0;
            };

            if (c > 9) {
                rem = c / 10;
                c = c % 10;
            }

            if (head == null) {
                head = new ListNode(c);
                last = head;
                if (la != null) la = la.next;
                if (lb != null) lb = lb.next;
                continue;
            }

            ListNode thisOne = new ListNode(c);
            last.next = thisOne;
            last = thisOne;

            if (la != null) la = la.next;
            if (lb != null) lb = lb.next;
        }
        return head;
    }
}
