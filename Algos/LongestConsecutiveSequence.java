/**
* Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.
* You must write an algorithm that runs in O(n) time.
* eg: Input: nums = [100,4,200,1,3,2] Output: 4
* Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
*
*/
class LongestConsecutiveSequence {
    // Longest Consecutive Sequence

    public int longestConsecutive(int[] nums) {
        Arrays.sort(nums);
        int maxSeq = 0;
        int thisSeq = 1;

        // System.out.println("Arr : " + Arrays.toString(nums));
        for(int i=0; i<nums.length-1; i++) {
            if(nums[i+1] == nums[i]) {
                // System.out.println("Same number, just continue");
                continue;
            } else if(nums[i+1] - nums[i] == 1) {
                // System.out.println("Incrementing at : " + i);
                thisSeq++;
            } else if(nums[i+1] - nums[i] > 1 ||
                      nums[i+1] - nums[i] < 1) {
                if(thisSeq > maxSeq) {
                    maxSeq = thisSeq;
                    thisSeq=1;
                }
            }
        }
        if(thisSeq > maxSeq) maxSeq = thisSeq;

        return maxSeq;
    }

}
