// Given an integer array nums, return true if any value appears more than once in the array, otherwise return false.

export function isDuplicate(arr: Array<number>): boolean {  // loop with index and check equalance
    
    for(let i=0; i<arr.length; i++) {
        for(let j=0; j<arr.length; j++) {
            if(i != j && arr[i] === arr[j]) return true;
        }
    }
    return false;
}

export function betterIsDuplicate(arr: Array<number>): boolean {    // convert to set, if length matches after that, its duplicate
    const setConverted: Set<number> = new Set(arr);
    return arr.length !== setConverted.size;
}