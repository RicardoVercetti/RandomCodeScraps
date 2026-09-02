package org.example.trees;

public class SubTreeOfAnotherTree {
    // Problem statement:
    // Given the roots of two binary trees root and subRoot, return true if there is a subtree of root with the same structure and node values of subRoot and false otherwise.
    // A subtree of a binary tree is a tree that consists of a node in tree and all of this node's descendants. The tree could also be considered as a subtree of itself.

    // Example 1:
    //             Root
    //              1
    //             /  \         Subroot
    //            2    3            2
    //           / \               / \
    //          4   5             4   5
    // Input: root = [1,2,3,4,5], subRoot = [2,4,5]
    // Output: true

    // Example 2:
    //             Root
    //              1
    //             /  \         Subroot
    //            2    3            2
    //           / \               / \
    //          4   5             4   5
    //         /
    //        6
    // Input: root = [1,2,3,4,5,null,null,6], subRoot = [2,4,5]
    // Output: false

    // Constraints:
    //
    //    The number of nodes in the root tree is in the range [1, 2000].
    //    The number of nodes in the subRoot tree is in the range [1, 1000].
    //    -10^4 <= root.val <= 10^4
    //    -10^4 <= subRoot.val <= 10^4

    public static void main(String[] args) {
        System.out.println("here goes nothing...");

        TreeNode t1 = new TreeNode(1);
        TreeNode t2 = new TreeNode(2);
        TreeNode t3 = new TreeNode(3);
        TreeNode t4 = new TreeNode(4);
        TreeNode t5 = new TreeNode(5);
        TreeNode t6 = new TreeNode(6);

        t1.left = t2;
        t1.right = t3;
        t2.left = t4;
        t2.right = t5;
        t4.left = t6;

        TreeNode s1 = new TreeNode(2);
        TreeNode s2 = new TreeNode(4);
        TreeNode s3 = new TreeNode(5);
        s1.left = s2;
        s1.right = s3;

        boolean result = isSubTreeOfAnotherTree(t1, s1);
        System.out.println("result: " + result);
    }

    public static boolean isSubTreeOfAnotherTree(TreeNode root, TreeNode subroot) {
        if (root == null) {
            return false;
        }

        boolean current = isCurrentTreeMatch(root, subroot);
        boolean left = isCurrentTreeMatch(root.left, subroot);
        boolean right = isCurrentTreeMatch(root.right, subroot);
        return current || left || right;
    }

    public static boolean isCurrentTreeMatch(TreeNode root, TreeNode subroot) {
        if (root == null && subroot == null) return true;
        if (root == null || subroot == null) return false;

        if (root.val == subroot.val) {
            boolean left = isCurrentTreeMatch(root.left, subroot.left);
            boolean right = isCurrentTreeMatch(root.right, subroot.right);
            return left && right;
        }
        return false;
    }
}
