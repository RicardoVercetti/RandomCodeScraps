package org.example.trees;

import java.util.LinkedList;
import java.util.Queue;

public class MaximumDepthOfBinaryTree {
    // Problem statement:
    // Given the root of a binary tree, return its depth.
    // The depth of a binary tree is defined as the number of nodes along the longest path from the root node down to the farthest leaf node.

    // Example 1:
    // Input: root = [1,2,3,null,null,4]
    // Output: 3
    // graph view:
    //         1
    //       /   \
    //      2     3
    //          /
    //         4

    // Example 2:
    // Input: root = []
    // Output: 0

    // Constraints:
    //    0 <= The number of nodes in the tree <= 100.
    //    -100 <= Node.val <= 100

    public static void main(String[] args) {
        System.out.println("here goes nothing...");
        TreeNode n1 = new TreeNode(1);
        TreeNode n2 = new TreeNode(2);
        TreeNode n3 = new TreeNode(3);
        TreeNode n4 = new TreeNode(4);
        n1.left = n2;
        n1.right = n3;
        n3.left = n4;

        int result = dfsRecursive(n1);
        System.out.println("result: " + result);
        int bfsResutlt = bfs(n1);
        System.out.println("bfs result: " + bfsResutlt);
    }

    public static int dfsRecursive(TreeNode root) {
        if (root == null) return 0;

        return 1 + Math.max(dfsRecursive(root.left), dfsRecursive(root.right));
    }

    public static int bfs(TreeNode root) {
        if (root == null) return 0;

        int level = 0;
        Queue<TreeNode> queue = new LinkedList<>();
        queue.offer(root);

        while (!queue.isEmpty()) {
            int levelSize = queue.size();
            for (int i=0; i<levelSize; i++) {
                TreeNode item = queue.poll();
                if (item == null) continue;

                if (item.left != null) queue.offer(item.left);
                if (item.right != null) queue.offer(item.right);
            }
            level ++;
        }

        return level;
    }
}
