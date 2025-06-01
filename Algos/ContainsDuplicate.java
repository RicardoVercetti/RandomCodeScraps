/**
* Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.
* Example 1:
* Input: nums = [1,2,3,1]
* Output: true
* Explanation: The element 1 occurs at the indices 0 and 3.
*/
class ContainsDuplicate {


	// the efficient way -> Don't loop through all the elements, skip the left that is already checked
	public boolean containsDuplicate(int[] nums) {
	        int currentIndex = 0;
	        int compIndex = currentIndex+1;
	        int maxlength = nums.length-1;
	        System.out.println("max : " + maxlength);
	        while(currentIndex<maxlength) {    // dont loop if curr is == lase
	            if(nums[currentIndex] == nums[compIndex]) {
	                return true;
	            }

	            if(compIndex == maxlength) {
	                currentIndex++;
	                compIndex = currentIndex +1;
	            } else if(compIndex < maxlength) {
	                compIndex++;
	            }
	        }
	        return false;
	}

}
