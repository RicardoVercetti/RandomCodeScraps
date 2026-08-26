package org.example.trees;

public class BalancedBinaryTree {
    // Problem statement:
    // Given a binary tree, return true if it is height-balanced and false otherwise.
    // A height-balanced binary tree is defined as a binary tree in which the left and right subtrees of every node differ in height by no more than 1.

    // Example 1:
    //      1
    //     / \
    //    2   3
    //       /
    //      4
    //
    // Input: root = [1,2,3,null,null,4]
    // Output: true

    // Example 2:
    //      1
    //     / \
    //    2   3
    //       /
    //      4
    //     /
    //    5
    //
    // Input: root = [1,2,3,null,null,4,null,5]
    // Output: false


    // Example 3:
    // Input: root = []
    // Output: true

    // Constraints:
    //
    //    The number of nodes in the tree is in the range [0, 1000].
    //    -1000 <= Node.val <= 1000

    public static void main(String[] args) {
        System.out.println("here goes nothing...");

        TreeNode n1 = new TreeNode(1);
        TreeNode n2 = new TreeNode(2);
        TreeNode n3 = new TreeNode(3);
        TreeNode n4 = new TreeNode(4);
        TreeNode n5 = new TreeNode(5);

        n1.left = n2;
        n1.right = n3;
        n3.left = n4;
        n4.left = n5;

        boolean out = findBool(n1);
        System.out.println("result: " + out);
    }

    public static int dfs(TreeNode root) {
        if (root == null) return 0;

        int left = dfs(root.left);
        int right = dfs(root.right);

        if (left == -1 || right == -1) return -1;
        int diff = Math.abs(left - right);
        if (diff > 1) return -1;
        return 1 + Math.max(left, right);
    }

    public static boolean findBool(TreeNode root) {
        int ret = dfs(root);
        return ret != -1;
    }
}
