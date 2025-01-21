package Java.Mastery.Algorithms;

public class MergeStringAlternatively {
    // 1768. Merge Strings Alternately
    // You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. 
    // If a string is longer than the other, append the additional letters onto the end of the merged string.

    // Example 1:

    // Input: word1 = "abc", word2 = "pqr"
    // Output: "apbqcr"
    // Explanation: The merged string will be merged as so:
    // word1:  a   b   c
    // word2:    p   q   r
    // merged: a p b q c r

    public static String mergeAlternately(String word1, String word2) {
        // take the biggest string length as the max length
        StringBuilder sb = new StringBuilder();
        int highestLength = word1.length() < word2.length() ? word2.length() : word1.length();
        for (int i=0; i<highestLength; i++) {
            if(i+1 <= word1.length()) {
                sb.append(word1.charAt(i));
            }

            if(i+1 <= word2.length()) {
                sb.append(word2.charAt(i));
            }
        }
        return  sb.toString();
    }

    public static void main(String[] args) {
        String s1 = "abc";
        String s2 = "efghi";
        System.out.println(mergeAlternately(s1, s2));

    }

}
