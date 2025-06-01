/**
* Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
* You may assume that each input would have exactly one solution, and you may not use the same element twice.
* You can return the answer in any order.
* Eg: Input: nums = [3,2,4], target = 6 Output: [1,2]
*/
class TwoSum {
	public int[] twoSum(int[] nums, int target) {
        for(int i=0; i<nums.length; i++) {
            for(int j=0; j<nums.length; j++) {
                if(i!=j && nums[i]+nums[j] == target) {
                    int[] result = {
                        i, j
                    };
                    return result;
                }
            }
        }

        return new int[]{0,1};

    	}
}
