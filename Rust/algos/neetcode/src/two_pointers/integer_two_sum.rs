// integer two sum II

// gives the array of integers in ascending order, return the array of two inices which gives the targer number when they are summed up
// index1 < index2 and there will always be one valid solution.
// its 1 indexed, meaning indexing of these items in array starts from 1..n;

// eg_1: Input: numbers = [1,2,3,4], target = 3             Output: [1,2]

#[allow(dead_code)]
fn two_integer_sum<const S: usize>(nums: &[i32; S], target: i32) -> [i32; 2] {
    let mut pointer_1:usize = 0;
    let mut pointer_2: usize = 1;
    println!("S: {}", S);

    for _i in 0..S {
        while pointer_2 < S {
            if target == nums[pointer_1] + nums[pointer_2] {
                // println!("found idx: {} || {}", pointer_1, pointer_2);
                return [pointer_1 as i32 + 1, pointer_2 as i32 + 1];
            }
        pointer_2 = pointer_2 + 1;
        // println!("Positions: {} || {}, i={}", pointer_1, pointer_2, i);
        }
        pointer_1 = pointer_1 + 1;
        pointer_2 = pointer_1 + 1;
        // println!("reset pointer_2 to: {}", pointer_2);
        // println!("reset pointer_1 to: {}", pointer_1);
    }

    [0, 0]
}

mod tests {

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        let nums = [1, 2, 3, 4];
        let target = 3;
        let result = two_integer_sum(&nums, target);
        assert_eq!(result, [1, 2]);
    }
}