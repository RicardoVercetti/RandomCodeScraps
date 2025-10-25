import { isAnagram } from "../../arrays_n_hashing/valid-anagram";

describe("brute force isAnagram", () => {
    it("positive anagram", () => {
        const first = "racecar";
        const second = "carrace";
        expect(isAnagram(first, second)).toBe(true);
    });

    it("negative anagram", () => {
        const first = "jar";
        const second = "jam";
        expect(isAnagram(first, second)).toBe(false);
    });
} )