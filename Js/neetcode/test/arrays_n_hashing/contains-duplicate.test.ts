import { isDuplicate, betterIsDuplicate } from "../../arrays_n_hashing/contains-duplicate";

describe("brute force contains duplicate", () => {
    it("positive case", () => {
        const nums = [1,2, 3, 3];
        expect(isDuplicate(nums)).toBe(true);
    });

    it("negative case", () => {
        const nums = [1, 2, 3, 4];
        expect(isDuplicate(nums)).toBe(false);
    })
});

describe("better contains duplicate", () => {
    it("positive case", () => {
        const nums = [1,2, 3, 3];
        expect(betterIsDuplicate(nums)).toBe(true);
    });

    it("negative case", () => {
        const nums = [1, 2, 3, 4];
        expect(betterIsDuplicate(nums)).toBe(false);
    })
});