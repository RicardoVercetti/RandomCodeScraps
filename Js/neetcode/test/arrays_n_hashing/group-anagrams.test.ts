import { groupAnagram } from "../../arrays_n_hashing/group-anagrams";

describe("group anagram", () => {
    it("test 1", () => {
        const strs = ["act","pots","tops","cat","stop","hat"];
        expect(groupAnagram(strs)).toStrictEqual([ [ 'act', 'cat' ], [ 'pots', 'tops', 'stop' ], [ 'hat' ] ])
    })
})