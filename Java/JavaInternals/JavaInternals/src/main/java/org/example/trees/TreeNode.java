package org.example.trees;

public class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;

    public TreeNode(int val) {
        this.val = val;
    }

    public TreeNode(int val, TreeNode left, TreeNode right) {
        this.val = val;
        this.left = left;
        this.right = right;
    }

    @Override
    public String toString() {
        return "(" + this.val + "), left=" + (this.left != null ? this.left.val : "null") + ", right=" + (this.right != null ? this.right.val : "null");
    }

//    public String toTreeString() {
//        StringBuilder sb = new StringBuilder();
//
//    }
}
