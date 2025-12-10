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

#[allow(dead_code)]
pub fn filter_overlapping_ranges(all_ranges: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let l_all_ranges = all_ranges.clone();
    let full_length = l_all_ranges.len();
    let mut new_array: Vec<(i64, i64)> = Vec::new();

    let mut continual_range: Option<(i64, i64)> = None;
    let mut step = 0;
    
    while step < full_length {
        // println!("step: {}", step);
        // println!("is_none: {}", continual_range.is_none());
        if continual_range.is_none() {
            continual_range = Some(l_all_ranges[step]);
        } else {
            // its some so this element have to be compared with the element in the continual range
            let (next_lower, next_upper) = l_all_ranges[step];
            let (this_lower, this_upper) = continual_range.unwrap();

            // println!("comparing this {this_lower}:{this_upper} with {next_lower}:{next_upper}");

            if  this_lower <= next_lower && this_upper >= next_upper {   // TL <= NL && TU >= NU
                // swallow → dont add                                  3-5  10-14 
                // println!("swallowing: {}/{}", next_lower, next_upper);

            } else if this_lower <= next_lower && next_lower <= this_upper && next_upper > this_upper {  // TL <= NL && NL <= TR && NR > TR               // 10-14  12-18
                // reformat the cont item's upper bound

                // println!("reformat for: {:?}", continual_range);
                // println!("this items: {}/{}", this_lower, this_upper);

                continual_range = Some((this_lower, next_upper));
                // println!("after reformat: {:?}", continual_range);
                
            } else if this_upper < next_lower {             // TR < NL  10-14  16-20
                // this is a new item, put the cont in list and add this in the cont
                new_array.push(continual_range.unwrap());
                continual_range = Some((next_lower, next_upper));
                // println!("pushed to new array: {:?}", new_array);
            } else {
                unimplemented!("this condition should not execute");
            }
        }

        step += 1;
    }

    if continual_range.is_some() {
        new_array.push(continual_range.unwrap());
    }

    new_array
}

#[allow(dead_code)]
pub fn count_fresh_ingredients(all_ranges: &Vec<(i64, i64)>) -> i64 {
    let mut count: i64 = 0;
    for (left, right) in all_ranges {
        let diff = *right - *left + 1;
        count += diff;
    }
    count
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

    // #[test]
    // fn test_acutal_data() {         // warn: this runs forever
    //     let sample_data = get_file_contents("attachment_data_d5.md");
    //     let all_fresh = find_all_fresh_in_ranges(&separate_data(&sample_data).0);
    //     assert_eq!(all_fresh.len(), 243);
    // }

    #[test]
    fn test_actual_data_faster() {
        let sample_data = get_file_contents("attachment_data_d5.md");
        let (mut ranges, _) = separate_data(&sample_data);
        ranges.sort_by_key(|f| f.0);
        let filtered = filter_overlapping_ranges(&ranges);
        let count = count_fresh_ingredients(&filtered);
        assert_eq!(count, 333892124923577);
    }
}