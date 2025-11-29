// product of array without self
    
    // for a list of numbers, return a list of arrays which gives the procuct of rest of the items expect the one that's in the position(num[i])
    // eg1: input nums = [1, 2, 4, 6], out = [48, 24, 12, 8]
    // eg2: input numms = [-1, 0, 1, 2, 3], out = [0, -6, 0, 0, 0]

#[allow(dead_code)]
pub fn product_of_array_without_self<const N: usize>(nums: &[i32; N]) -> [i32; N] {
    let mut ret_nums: [i32; N] = [0; N];
    for (index, _) in nums.iter().enumerate() {
        let mut mult = 1;
        for (inner_index, second_item) in nums.iter().enumerate() {
            if index == inner_index {
                continue;
            }
            mult = mult * second_item;
        }
        ret_nums[index] = mult;
    }
    ret_nums
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        // test brute force in sample 1
        let nums =  [1, 2, 4, 6];
        let result = product_of_array_without_self(&nums);

        assert_eq!(result, [48, 24, 12, 8]);
    }

    #[test]
    fn test_2() {
        // test brute force in sample 2
        let nums = [-1, 0, 1, 2, 3];
        let result = product_of_array_without_self(&nums);
        assert_eq!(result, [0, -6, 0, 0, 0]);
    }
}