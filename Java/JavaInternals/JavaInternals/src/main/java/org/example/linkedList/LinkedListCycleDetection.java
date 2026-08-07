package org.example.linkedList;

public class LinkedListCycleDetection {
    // Problem statement:
    // Given the beginning of a linked list head, return true if there is a cycle in the linked list. Otherwise, return false.
    // There is a cycle in a linked list if at least one node in the list can be visited again by following the next pointer.
    // Internally, index determines the index of the beginning of the cycle, if it exists. The tail node of the list will set it's next pointer to the index-th node. If index = -1, then the tail node points to null and no cycle exists.
    // Note: index is not given to you as a parameter.

    // Example 1:
    // Input: head = [1,2,3,4], index = 1
    // Output: true
    // Explanation: There is a cycle in the linked list, where the tail connects to the 1st node (0-indexed).

    // Example 2:
    // Input: head = [1,2], index = -1
    // Output: false

    // Constraints:
    //    0 <= Length of the list <= 1000.
    //    -1000 <= Node.val <= 1000
    //    index is -1 or a valid index in the linked list.

    public static void main(String[] args) {
        System.out.println("here goes nothing...");
        ListNode n1 = new ListNode(1);
        ListNode n2 = new ListNode(2);
        ListNode n3 = new ListNode(3);
        ListNode n4 = new ListNode(4);

        n1.next = n2;
        n2.next = n3;
        n3.next = n4;
//        n4.next = n2;

        boolean output = solution(n1);
        System.out.println("soln: " + output);

    }

    public static boolean solution(ListNode head) {
        // approach, show pointer moves by 1 in each run, fast pointer moves by 2 positions.
        // if there is a loop, the movement goes on and on, and at some point(by n), the fast reaches slow pointer
        ListNode slow = head;
        ListNode fast = head.next.next;

        while (fast != null && fast.next != null) {
            if (fast == slow) return true;
            slow = slow.next;
            fast = fast.next.next;
        }

        return false;
    }

    public static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode nextNode) {
            this.val = val;
            this.next = nextNode;
        }
    }
}
