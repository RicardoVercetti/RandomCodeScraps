package org.example.linkedList;

import java.util.ArrayList;

public class ListNode {
    int val;
    ListNode next;

    public ListNode(int val) {
        this.val = val;
    }

    public ListNode(int val, ListNode next) {
        this.val = val;
        this.next = next;
    }

    public ArrayList<Integer> showAll() {
        ArrayList<Integer> arr = new ArrayList<>();
        ListNode head = this;
        while (head != null) {
            arr.add(head.val);
            head = head.next;
        }

        return arr;
    }

    @Override
    public String toString() {
        return "(" + this.val + ")";
    }
}
