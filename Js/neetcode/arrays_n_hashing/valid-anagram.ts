// An anagram is a string that contains the exact same characters as another string, but the order of the characters can be different.

// length must be equal, and remove the same letter for each letter from one array, by the end both arrays have to be empty
export function isAnagram(s: string, t: string): boolean {
    let l1 = s.split('');
    let l2 = t.split('');
    if(l1.length !== l2.length) return false;

    for(const letter of l1) {
        const i = l2.indexOf(letter);
        if(i === -1) continue;
        l2.splice(i, 1);
    }

    return l2.length === 0;
}