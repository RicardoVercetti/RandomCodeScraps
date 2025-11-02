// k-most frequent elements

// Given an integer array nums and an integer k, return the k most frequent elements within the array.

// Example 1:
// Input: nums = [1,2,2,3,3,3], k = 2
// Output: [2,3]

// Example 2:
// Input: nums = [7,7], k = 1
// Output: [7]

// Idea:
// go through each element
// check if its already in hashmap key, if yes add the count in the value
// if not, add the element to key and 1 to value.
// finally loop through each key n value, wherever the value is grater than or equal to k, add the key to array and return it.

export const kMostFrequentElement = (arr: number[], k: number): number[] => {
    const hashmap = new Map<number, number>();
    for(const item of arr) {
        const ith = hashmap.get(item);
        if(ith) {
            hashmap.set(item, ith+1);
        } else {
            hashmap.set(item, 1);
        }
    }
    return [...hashmap].filter(([_, values]) => values >= k).map(([key]) => key);
}