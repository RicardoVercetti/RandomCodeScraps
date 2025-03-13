// Given an array of numbers and a target value, return the indices of two numbers that add up to the target.

// Example Input: nums = [2, 7, 11, 15], target = 9
// Example Output: [0, 1] 

let arr = [2, 7, 11, 15];

function findIndices(arr: number[], target: number): number[]|undefined {
    for(let j=0; j<arr.length; j++) {
        // console.log(`Index: ${j}, Value: ${arr[j]}`);
        for (let i=0; i<arr.length; i++) {
            if(i<=j) {
                continue; 
            }
            if(arr[j] + arr[i] === target) {
                console.log(`comparing ${arr[j]} and ${arr[i]}`);
                return [arr[j], i];
            }
        }
    };
    return undefined;
}

console.log(findIndices([2, 7, 11, 15], 9));
