/**
* -- Top K Frequent Elements --
* Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.
* Eg 1: Input: nums = [1,1,1,2,2,3], k = 2 Output: [1,2]
* Eg 2: Input: nums = [1], k = 1 Output: [1]
*/
class TopKFrequent {
    public int[] topKFrequent(int[] nums, int k) {
        List<Integer> output = new ArrayList<>();
        int kNum = 0; 
        for(int i: nums) {
            if(k==i) {
                kNum++;
                output.add(kNum);
            }
        }
        return toNativeInt(output);
    }

    public int[] toNativeInt(List<Integer> arr) {
        int[] intArray = new int[arr.size()];

        for (int i = 0; i < arr.size(); i++) {
            intArray[i] = arr.get(i); // auto-unboxing
        }
        return intArray;
    }
}
