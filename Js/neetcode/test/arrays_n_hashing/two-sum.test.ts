import { twoSum } from "../../arrays_n_hashing/two-sum";

describe("two sum", () => {
    it("brute force sample one", () => {
        const nums = [3,4,5,6];
        const target = 7;
        expect(twoSum(nums, target)).toStrictEqual([0, 1]);
    });

    it("brute force sample two", () => {
        const nums = [4,5,6];
        const target = 10;
        expect(twoSum(nums, target)).toStrictEqual([0,2]);
    });

    it("brute force sample three", () => {
        const nums = [5,5];
        const target = 10;
        expect(twoSum(nums, target)).toStrictEqual([0, 1]);
    })
})