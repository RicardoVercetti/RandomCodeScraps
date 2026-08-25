package org.example.trees;

public class DiameterOfBinaryTree {
    // Problem statement:
    // The diameter of a binary tree is defined as the length of the longest path between any two nodes within the tree. The path does not necessarily have to pass through the root.
    // The length of a path between two nodes in a binary tree is the number of edges between the nodes. Note that the path can not include the same node twice.
    // Given the root of a binary tree root, return the diameter of the tree.

    // Example 1:
    // Input: root = [1,null,2,3,4,5]
    // visual:
    //       1
    //        \
    //         2
    //        /  \
    //       3    4
    //      /
    //     5
    //
    // Output: 3
    // Explanation: 3 is the length of the path [1,2,3,5] or [5,3,2,4].

    // Example 2:
    // Input: root = [1,2,3]
    // Output: 2

    // Constraints:
    //    1 <= number of nodes in the tree <= 100
    //    -100 <= Node.val <= 100

    static int res = 0;

    public static void main(String[] args) {
        System.out.println("here goes nothing...");
        TreeNode n1 = new TreeNode(1);
        TreeNode n2 = new TreeNode(2);
        TreeNode n3 = new TreeNode(3);
//        TreeNode n4 = new TreeNode(4);
//        TreeNode n5 = new TreeNode(5);

        // case 1:
//        n1.right = n2;
//        n2.left = n3;
//        n2.right = n4;
//        n3.left = n5;

        // case 2:
        n1.left = n2;
        n1.right = n3;

        int result = find(n1);
        System.out.println("res: " + result);
    }

    public static int dfs(TreeNode root) {
        if (root == null) return 0;

        int left = dfs(root.left);
        int right = dfs(root.right);

        res = Math.max(res, left+right);
        return 1 + Math.max(left, right);
    }

    public static int find(TreeNode root) {
        dfs(root);
        return res;
    }
}
