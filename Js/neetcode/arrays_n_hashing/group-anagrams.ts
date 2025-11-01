// Given an array of strings strs, group all anagrams together into sublists. You may return the output in any order.
// An anagram is a string that contains the exact same characters as another string, but the order of the characters can be different.


// loop by each element 
// take one, convert to list, sort and join
// have a hashmap and check if this string is in the hashmap
// if it exists, add this to one of the values, else add it as the first element in the collection
// finally go by one hashmap item at a time and combine all in a list

export const groupAnagram = (arr: string[]): string[][] => {
    const hashmap = new Map<string, string[]>();
    for(const one of arr) {
        const comparable = one.split('').sort().join("");
        if(hashmap.has(comparable)) {
            hashmap.get(comparable)?.push(one)
        } else {
            hashmap.set(comparable, [one]);
        }
    }

    let li: string[][] = []; 
    hashmap.forEach((val, key) => {
        li.push(val)
    })

    return li;
}