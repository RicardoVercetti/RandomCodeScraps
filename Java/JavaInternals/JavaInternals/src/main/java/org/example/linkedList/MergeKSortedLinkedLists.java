package org.example.linkedList;

import java.util.ArrayList;
import java.util.Stack;

public class MergeKSortedLinkedLists {
    // Problem statement:
    // You are given an array of k linked lists, where each list is sorted in ascending order.
    // Return the sorted linked list that is the result of merging all the individual linked lists.

    // Example 1:
    // Input: lists = [[1,2,4],[1,3,5],[3,6]]
    // Output: [1,1,2,3,3,4,5,6]

    // Example 2:
    // Input: lists = []
    // Output: []

    // Example 3:
    // Input: lists = [[]]
    // Output: []

    // Constraints:
    //    0 <= lists.length <= 10000
    //    0 <= lists[i].length <= 500
    //    -10000 <= lists[i][j] <= 10000
    //    lists[i] is sorted in ascending order.
    //    The sum of lists[i].length will not exceed 10000.

    public static void main(String[] args) {
        System.out.println("Another one...");

        ListNode la1 = new ListNode(1);
        ListNode la2 = new ListNode(2);
        ListNode la3 = new ListNode(4);
//        ListNode la4 = new ListNode(4);

        la1.next = la2;
        la2.next = la3;
//        la3.next = la4;

        ListNode lb1 = new ListNode(1);
        ListNode lb2 = new ListNode(3);
        ListNode lb3 = new ListNode(5);
//        ListNode lb4 = new ListNode(8);

        lb1.next = lb2;
        lb2.next = lb3;
//        lb3.next = lb4;


        ListNode lc1 = new ListNode(3);
        ListNode lc2 = new ListNode(6);

        lc1.next = lc2;

        ArrayList<ListNode> arr = new ArrayList<>();
        arr.add(la1);
        arr.add(lb1);
        arr.add(lc1);

//        System.out.println("from la1: " + linkedListString(la1));
//        System.out.println("from lb1: " + linkedListString(lb1));
//        ListNode combined = mergeTwoSortedLists(la1, lb1);
//        System.out.println("combined: " + linkedListString(combined));

        ListNode combined = mergeKSortedLinkedLists(arr);
        System.out.println("combined: " + linkedListString(combined));

    }

    public static ListNode mergeKSortedLinkedLists(ArrayList<ListNode> array) {
        Stack<ListNode> listNodeStack = new Stack<>();
        for (int i=0; i<array.size(); i+=2) {
            ListNode first = array.get(i);
            ListNode second = i+1 >= array.size() ? null : array.get(i+1);

            ListNode combined = mergeTwoSortedLists(first, second);
            listNodeStack.add(combined);
        }

        while (listNodeStack.size() > 1) {
            ListNode first = listNodeStack.pop();
            ListNode second = listNodeStack.pop();
            ListNode combined = mergeTwoSortedLists(first, second);
            listNodeStack.add(combined);
        }


        return listNodeStack.getFirst();
    }

    public static ListNode mergeTwoSortedLists(ListNode ln1, ListNode ln2) {
        ListNode head = null;
        ListNode tail = new ListNode(0);

        ListNode node1 = ln1;
        ListNode node2 = ln2;
        boolean isBothNull = (node1 == null && node2 == null);

        while (!isBothNull) {
            if (node1 != null && node2 != null) {
                if (node1.val < node2.val) {
                    tail.next = node1;
                    node1 = node1.next;
                    tail.next.next = null;
                } else {
                    tail.next = node2;
                    node2 = node2.next;
                    tail.next.next = null;
                }
            } else if (node1 != null && node2 == null) {
                tail.next = node1;
                node1 = node1.next;
                tail.next.next = null;
            } else if (node2 != null && node1 == null) {
                tail.next = node2;
                node2 = node2.next;
                tail.next.next = null;
            }
            if (head == null) head = tail.next;
            if (node1 == null && node2 == null) isBothNull = true;

            tail = tail.next;
        }
        return head;
    }

    public static String linkedListString(ListNode head) {
        StringBuilder sb = new StringBuilder();
        sb.append("(");
        while (head != null) {
            sb.append("\n( val=" + head.val + ", next: " + (head.next != null ? head.next.val : "null") + "),");
            head = head.next;
        }
        sb.append("\n)");
        return sb.toString();
    }
}
