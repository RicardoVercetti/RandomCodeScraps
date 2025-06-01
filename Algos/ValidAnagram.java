class ValidAnagram {
	// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
	// Example 1:
	// Input: s = "anagram", t = "nagaram"
	// Output: true
	public boolean isAnagram(String s, String t) {
        char[] sArr = s.toCharArray();
        char[] tArr = t.toCharArray();

        Arrays.sort(sArr);
        Arrays.sort(tArr);

        String newStringS = new String(sArr);
        String newStringT = new String(tArr);

        if(newStringS.equals(newStringT)) return true;
        return false;
    }
}
