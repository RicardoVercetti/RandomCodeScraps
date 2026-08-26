package org.example.trees;

public class SameBinaryTree {
    // Problem statement:
    // Given the roots of two binary trees p and q, return true if the trees are equivalent, otherwise return false.
    // Two binary trees are considered equivalent if they share the exact same structure and the nodes have the same values.

    // Example 1:
    // Input: p = [1,2,3], q = [1,2,3]
    // Output: true

    // Example 2:
    // Input: p = [4,7], q = [4,null,7]
    // Output: false

    // Example 3:
    // Input: p = [1,2,3], q = [1,3,2]
    // Output: false

    // Constraints:
    //
    //    0 <= The number of nodes in both trees <= 100.
    //    -100 <= Node.val <= 100

    public static void main(String[] args) {
        System.out.println("here goes nothing...");

        TreeNode n1 = new TreeNode(1);
        TreeNode n2 = new TreeNode(2);
        TreeNode n3 = new TreeNode(3);
        n1.left = n2;
        n1.right = n3;

        TreeNode n4 = new TreeNode(1);
        TreeNode n5 = new TreeNode(2);
        TreeNode n6 = new TreeNode(3);
        n4.left = n5;
        n4.right = n6;

        boolean result = isSameTree(n1, n4);
        System.out.println("result: " + result);

        TreeNode n7 = new TreeNode(4);
        TreeNode n8 = new TreeNode(7);
        n7.left = n8;

        TreeNode n9 = new TreeNode(4);
        TreeNode n10 = new TreeNode(7);
        n9.right = n10;

        boolean result2 = isSameTree(n7, n9);
        System.out.println("result2: " + result2);

        TreeNode n11 = new TreeNode(1);
        TreeNode n12 = new TreeNode(2);
        TreeNode n13 = new TreeNode(3);
        n11.left = n12;
        n11.right = n13;


        TreeNode n14 = new TreeNode(1);
        TreeNode n15 = new TreeNode(2);
        TreeNode n16 = new TreeNode(3);
        n14.left = n16;
        n14.right = n15;
        boolean result3 = isSameTree(n11, n14);
        System.out.println("result3: " + result3);

    }

    public static boolean isSameTree(TreeNode p, TreeNode q) {
        if (p == null || q == null) {
            return p == q;
        }

        boolean both = p.val == q.val;
        boolean leftBoolean = isSameTree(p.left, q.left);
        boolean rightBoolean = isSameTree(p.right, q.right);


        return both && leftBoolean && rightBoolean;
    }
}
