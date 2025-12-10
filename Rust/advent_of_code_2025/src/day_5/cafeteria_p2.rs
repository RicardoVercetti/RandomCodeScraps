use std::collections::BTreeSet;

#[allow(dead_code)]
pub fn find_all_fresh_in_ranges(ranges: &Vec<(i64, i64)>) -> BTreeSet<i64> {
    let mut set: BTreeSet<i64> = BTreeSet::new();

    for (left, right) in ranges {
        // println!("\rchecking range: {}:{}", left, right);
        for i in *left..=*right {
            set.insert(i);
        }
    }
    set
}


// attempt 2
// total number of fresh ingrediants are whats required
// for sample:
// 3-5                      # 5-3 = 2 + 1 = 3 fresh items
// 10-14                    # 14-10 = 4 + 1 = 5 fresh items
// 16-20                    # 20-16 = 4 + 1 = 5 fresh items
// 12-18                    # 18-12 = 6 + 1 = 7 fresh items

// + 1 on each is there cuz the ranges are inclusive of upper and lower bounds
// 3 + 5 + 5 + 7 = 20. but the range 12-18 is covered in 10-14 & 16-20 ranges

// soln:
// join all the overlapping ranges
// then subract the lower bound from upper bound, then +1
// add up all the results together, tada!!! its the result ;)

// approach for joining overlapping ranges:
// sort the array in ascending by the lower bound range
// loop by each item in array and compare it with the next item, there could be 3 different cases
// 1. the lower bound is in between the range and the upper bound is also inbetween the range → swallow it completely
// 2. the lower bound is in between the range but the upper bound is not in between, its higher → swallow the lower bound and reset the higher bound of the current item
// 3. the lower bound is higher than the upper bound of the item → eat 5 star and do nothing

pub fn filter_overlapping_ranges(all_ranges: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let l_all_ranges = all_ranges.clone();
    let full_length = l_all_ranges.len();
    let mut new_array: Vec<(i64, i64)> = Vec::new();

    let mut continual_range: Option<(i64, i64)> = None;
    let mut step = 0;
    
    while step < full_length {
        println!("step: {}", step);
        println!("is_none: {}", continual_range.is_none());
        if continual_range.is_none() {
            continual_range = Some(l_all_ranges[step]);
        } else {
            // its some so this element have to be compared with the element in the continual range
            let (this_lower, this_upper) = l_all_ranges[step];
            let (cont_lower, cont_upper) = continual_range.unwrap();

            if is_between(cont_lower, cont_upper, this_lower, this_upper) {
                // swallow → dont add
                println!("swallowing: {}/{}", this_lower, this_upper);

            } else if cont_lower <= this_lower && cont_upper >= this_lower && this_upper > cont_upper {
                // reformat the cont item's upper bound

                println!("reformat for: {:?}", continual_range);
                println!("this items: {}/{}", this_lower, this_upper);

                continual_range = Some((cont_lower, this_upper));
                
            } else if cont_upper < this_lower {
                // this is a new item, put the cont in list and add this in the cont
                new_array.push(continual_range.unwrap());
                continual_range = None;
                println!("pushed to new array...");
            } else {
                println!("Unexpected condition...");
            }
        }

        step += 1;
    }

    new_array
}

fn is_between(item_lower: i64, item_upper: i64, next_lower: i64, next_upper: i64) -> bool {
    item_lower <= next_lower  && next_lower < next_upper && item_upper >= next_upper
}

mod tests {
    #[allow(unused_imports)]
    use crate::{day_5::cafeteria_p1::separate_data, utils::get_file_contents};

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn sample_test() {
        let sample = "3-5
                        10-14
                        16-20
                        12-18

                        1
                        5
                        8
                        11
                        17
                        32";
        let (range, _) = separate_data(&sample);
        let all_fresh = find_all_fresh_in_ranges(&range);
        assert_eq!(all_fresh.len(), 14);
    }

    #[test]
    fn test_acutal_data() {
        let sample_data = get_file_contents("attachment_data_d5.md");
        let all_fresh = find_all_fresh_in_ranges(&separate_data(&sample_data).0);
        assert_eq!(all_fresh.len(), 243);
    }
}