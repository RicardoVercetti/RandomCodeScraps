// Arrays & Hashing > longest consecutive sequence

// given an array of integers, longest consecutive sequence would be the longest continuous counting that one can do in sequence.
// for li = [2, 20, 4, 10, 3, 4, 5] the longest consecutive would be 4 → 2, 3, 4, 5

// Constraints:
// 1. 0 <= nums.length <= 1000
// 2. -10^9 <= nums[i] <= 10^9
// recommended time & space complexity: O(n) time & space complexity, n is the size of input array

// steps:
// 1. Sort the array first(and optionally remove duplicates)
// 2. count from left most, check the num[n-1] - num[n], if this is 1, add the count
// 3. have max_count, reset count when the above condition doesn't get satisfied in any loop
// 4. finally return the max_count

#[allow(unused_imports)]
use std::vec;

#[allow(dead_code)]
fn remove_duplicates(li: Vec<i32>) -> Vec<i32> {
    let mut re = Vec::new();
    // loop for each, check if this element exists in the new list, if no, push it to the new list
    for i in li {
        if !re.contains(&i) {
            re.push(i);
        }
    }

    re
}

#[allow(dead_code)]
pub fn longest_consecutive(list: &mut Vec<i32>) -> i32 {
    let mut max_count = 0;
    let mut count = 0;

    let mut set = remove_duplicates(list.clone());
    let _ = set.sort_unstable();
    // println!("sorted: {:?}", set);

    let mut i = 0;
    let last = set.len();
    loop {
        // if i == 0 {
        //     count = 1;
        // }
        if i != last-1 {
            let this_item = &set[i];
            let next_item = &set[i+1];

            if next_item - this_item == 1 && next_item != this_item {
                if i == 0 {
                    count += 2;
                } else {
                    count += 1;
                }
                // println!("curr: {}, next: {}, count: {}", this_item, next_item, count);
            } else {
                if count > max_count {
                    max_count = count.clone();
                }
                count = 0;
                // println!("resetting count on this: {}, next: {}", this_item, next_item);
            }
            i += 1;
        } else {
            if count > max_count {
                max_count = count.clone();
            }
            break;
        }

    }

    max_count
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        let mut  li = vec![2, 20, 4, 10, 3, 4, 5];
        // let mut li = vec![0,3,2,5,4,6,1,1];
        let seq = longest_consecutive(&mut li);

        assert_eq!(seq, 4);
    }

    #[test]
    fn test_2() {
        let mut li = vec![0,3,2,5,4,6,1,1];
        let seq = longest_consecutive(&mut li);

        assert_eq!(seq, 7);
    }

}