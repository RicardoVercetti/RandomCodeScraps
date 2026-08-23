package org.example.linkedList;

public class ReverseNodesInKGroup {
    // Problem statement:
    // You are given the head of a singly linked list head and a positive integer k.
    // You must reverse the first k nodes in the linked list, and then reverse the next k nodes, and so on. If there are fewer than k nodes left, leave the nodes as they are.
    // Return the modified list after reversing the nodes in each group of k.
    // You are only allowed to modify the nodes' next pointers, not the values of the nodes.

    // Example 1:
    // Input: head = [1,2,3,4,5,6], k = 3
    // Output: [3,2,1,6,5,4]

    // Example 2:
    // Input: head = [1,2,3,4,5], k = 3
    // Output: [3,2,1,4,5]

    // Constraints:
    //    The length of the linked list is n.
    //    1 <= k <= n <= 5000
    //    0 <= Node.val <= 100

    public static void main(String[] args) {
        System.out.println("reverse this..");
        ListNode ln1 = new ListNode(1);
        ListNode ln2 = new ListNode(2);
        ListNode ln3 = new ListNode(3);
        ListNode ln4 = new ListNode(4);
        ListNode ln5 = new ListNode(5);
//        ListNode ln6 = new ListNode(6);
//        ListNode ln7 = new ListNode(7);

        ln1.next = ln2;
        ln2.next = ln3;
        ln3.next = ln4;
        ln4.next = ln5;
//        ln5.next = ln6;
//        ln6.next = ln7;

        System.out.println("str: " + nodeToString(ln1));

        ListNode headAfterKGroupReversal = reversingInKGroup(ln1, 3);
        System.out.println("reversed: " + nodeToString(headAfterKGroupReversal));
    }

    public static ListNode reversingInKGroup(ListNode head, int k) {
        // first lets try specifically reversing one section

        ListNode tail = head;
        ListNode dummy = new ListNode(0);
        dummy.next = head;
        ListNode tailm1 = dummy;

        while (tail != null) {

            HeadAndTail headAndTail = specificallyReverse(tailm1, k);
            if (headAndTail == null) break;
            tailm1 = headAndTail.tail;
        }

        return dummy.next;
    }

    /// reverse from the head node(inclusive) to kth node(inclusive)
    public static HeadAndTail specificallyReverse(ListNode headm1, int k) {
        ListNode head = headm1.next;
        ListNode end = getEnd(head, k);

        if (end == null) return null;
        ListNode endp1 = end.next;

        ListNode segHead = revSegment(head, endp1);
        headm1.next = segHead;
        head.next = endp1;
        return new HeadAndTail(segHead, head);
    }

    public static class HeadAndTail {
        ListNode head;
        ListNode tail;

        public HeadAndTail(ListNode head, ListNode tail) {
            this.head = head;
            this.tail = tail;
        }

        public ListNode getHead() {
            return this.head;
        }

        public ListNode getTail() {
            return this.tail;
        }
    }

    public static ListNode revSegment(ListNode head, ListNode endP1) {
        ListNode curr = head;
        ListNode prev = null;

        while(curr != null && curr != endP1) {
            ListNode temp = curr.next;
            curr.next = prev;
            prev = curr;
            curr = temp;
        }
        return prev;
    }

    public static ListNode getEnd(ListNode head, int k) {
        int n = 1;
        ListNode tail = head;

        while (n < k && tail != null) {
            tail = tail.next;
            n++;
        }

        if (n < k) return null;
        return tail;
    }

    public static ListNode letsTryReversingIt(ListNode head) {
        ListNode prev = null;
        ListNode current = head;

        while (current != null) {
            ListNode temp = current.next;
            current.next = prev;
            prev = current;
            current = temp;
        }

        return prev;
    }

    public static String nodeToString(ListNode head) {
        StringBuilder sb = new StringBuilder();
        sb.append("(\n");
        while (head != null) {
            sb.append("val: " + head.val + "),\n");
            head = head.next;
        }
        sb.append(")");
        return sb.toString();
    }
}
