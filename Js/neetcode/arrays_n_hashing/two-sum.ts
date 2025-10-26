// Given an array of integers nums and an integer target, return the indices i and j such that nums[i] + nums[j] == target and i != j.
// You may assume that every input has exactly one pair of indices i and j that satisfy the condition.
// Return the answer with the smaller index first.

export function twoSum(nums: number[], target: number): [number, number] | undefined {
    for(let i=0; i<nums.length; i++) {
        for(let j=0; j<nums.length; j++) {
            if((nums[i]! + nums[j]!) === target && i !== j) {
                return [i, j];
            }
        }
    }

    return undefined;
}