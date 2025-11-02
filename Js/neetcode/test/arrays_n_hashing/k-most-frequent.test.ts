import { kMostFrequentElement } from "../../arrays_n_hashing/k-most-frequent";

describe("k-most frequent elements", () => {
    it("brute force solution test - 1", () => {
        expect(kMostFrequentElement([1,2,2,3,3,3], 2)).toStrictEqual([2, 3]);
    });

    it("brute force solution test - 2", () => {
        expect(kMostFrequentElement([7,7], 1)).toStrictEqual([7]);
    });
});