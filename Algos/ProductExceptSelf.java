/**
* Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
* The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
* You must write an algorithm that runs in O(n) time and without using the division operation.
* Eg: Input: nums = [1,2,3,4] Output: [24,12,8,6]
*/
class ProductExceptSelf {
// Product of Array Except Self
    public int[] productExceptSelf(int[] nums) {
	int[] products = new int[nums.length];
	for(int i=0; i<nums.length; i++) {
	    int prod = 1;
	    for(int j=0; j<nums.length; j++) {
	        if(i!=j) {
	            prod *= nums[j];
	        }

	    }
	    products[i] = prod;
	}
	return products;
    }

}
