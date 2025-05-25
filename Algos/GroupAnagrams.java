public class GroupAnagrams {
    /**
     * Group similar anagrams together
     */
    static ArrayList<ArrayList<String>> groupAnagrams(String[] arr) {
        // String[] newArr = arr.clone();

        ArrayList<ArrayList<String>> allGroups = new ArrayList<>();

        Map<String, ArrayList<String>> mp = new HashMap<>();

        for (String el : arr) {
            char[] charArr = el.toCharArray();
            Arrays.sort(charArr);
            String sortedStr = new String(charArr);
            if (mp.containsKey(sortedStr)) {
                mp.get(sortedStr).add(el);
                continue;
            }
            ArrayList<String> grpArr = new ArrayList<String>();
            grpArr.add(el);
            mp.put(sortedStr, grpArr);
        }

        mp.forEach((key, val) -> {
            allGroups.add(val);
        });

        return allGroups;
    }

}
