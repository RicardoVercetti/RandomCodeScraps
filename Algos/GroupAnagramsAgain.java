/**
* Given an array of strings strs, group the anagrams together. You can return the answer in any order.
* Eg: 
* Input: strs = ["eat","tea","tan","ate","nat","bat"]
* Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
*/
class GroupAnagramsAgain {
	public List<List<String>> groupAnagrams(String[] strs) {
        Map<String, ArrayList<String>> map = new HashMap<>();
        for(String s: strs) {
            char[] c = s.toCharArray();
            Arrays.sort(c);
            String theSorted = new String(c);
            if(map.containsKey(theSorted)) {
                map.get(theSorted).add(s);
            } else {
                ArrayList<String> arr = new ArrayList();
                arr.add(s);
                map.put(theSorted, arr);
            }
        }
        List<List<String>> list = new ArrayList<>();
        map.forEach((k, v) -> {
            list.add(v);
        });

        return list;
    }
}
