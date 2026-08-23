package org.example.trees;

public class InvertBinaryTree {
    // Problem statement:
    // You are given the root of a binary tree root. Invert the binary tree and return its root.

    // Example 1:
    // Input: root = [1,2,3,4,5,6,7]
    // Output: [1,3,2,7,6,5,4]

    // Example 2:
    // Input: root = [3,2,1]
    // Output: [3,1,2]

    // visual representation:
    //      3
    //    /   \
    //   1     2

    // Example 3:
    // Input: root = []
    // Output: []

    // Constraints:
    //    0 <= The number of nodes in the tree <= 100.
    //    -100 <= Node.val <= 100

    public static void main(String[] args) {
        System.out.println("here goes nothing...");
        TreeNode first = new TreeNode(1);
        TreeNode second = new TreeNode(2);
        TreeNode third = new TreeNode(3);
        TreeNode fourth = new TreeNode(4);
        TreeNode fifth = new TreeNode(5);
        TreeNode sixth = new TreeNode(6);
        TreeNode seventh = new TreeNode(7);

        first.left = second;
        first.right = third;

        second.left = fourth;
        second.right = fifth;

        third.left = sixth;
        third.right = seventh;

        System.out.println("middle: " + first);
        System.out.println("second: " + second);
        System.out.println("third: " + third);
        invertRecursively(first);
        System.out.println("--- after recursive inverting ---");
        System.out.println("middle: " + first);
        System.out.println("second: " + second);
        System.out.println("third: " + third);
    }

    public static void invertRecursively(TreeNode root) {
        if (root == null) return;

        TreeNode temp = root.left;
        root.left = root.right;
        root.right = temp;

        invertRecursively(root.left);
        invertRecursively(root.right);
    }
}
