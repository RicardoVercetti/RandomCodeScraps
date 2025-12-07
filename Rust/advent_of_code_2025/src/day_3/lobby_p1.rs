
#[allow(dead_code)]
fn max_two_of_bank(one_bank: &String) -> (i32, i32) {
    // have max_one and max_two variables & max_one_pos and max_two_pos as well and initiate all to 0
    // loop through each item with index number, set max_one and max_one_pos if item is bigger than max_one
    // when item is smaller or same to max_one and max_one_pos is not equal to the pos of item, and max_two is lesser than item, assign item to max_two and pos to max_two_pos
    // whenever max_one gets reassigned, also reassign max_two by checking if max_two is greater than max_one and also reassign max_two_pos from max_one_pos

    let mut max_one = 0;
    let mut max_one_pos: usize = 0;
    let mut max_two = 0;
    let mut max_two_pos: usize = 0;

    for (i, item) in one_bank.chars().enumerate() {
        let item_int = item.to_digit(10).expect(&format!("invalid digit: {}", item));
        
        // check for first item
        if item_int >= max_one {
            
            // when max_one gets reassigned, assign max_two too
            if max_two < max_one && max_one_pos != max_two_pos {
                max_two = max_one;
                max_two_pos = max_one_pos;
                // println!("second gets reassigned cuz of first: {}, {}", max_one_pos, max_one);
            }

            // assign/reassign max_one and max_one_pos
            max_one = item_int;
            max_one_pos = i;
            // println!("first item gets assigned: {}, {}", i, item_int);
        }

        // check for second item
        if item_int <= max_one && item_int >= max_two && max_one_pos != i {
            max_two = item_int;
            max_two_pos = i;
            // println!("second item gets reassigned to: {}, {}", i, item_int);
        }
    }

    // return the tuple of max_one and max_two
    if max_one_pos < max_two_pos {
        (max_one as i32, max_two as i32)
    } else {
        (max_two as i32, max_one as i32)
    }
    
}

#[allow(dead_code)]
fn nah_id_brute_force_it(inp: &String) -> i32 {
    let mut max = 0;

    for (i, item_i) in inp.chars().enumerate() {
        for (j, item_j) in inp.chars().enumerate() {
            if i != j && j > i {
                // make int and assign it to max
                let max_candidate: i32 = format!("{item_i}{item_j}").parse().unwrap();
                if max_candidate > max {
                    max = max_candidate;
                }
            }
        }
    }
    max
}

fn find_sum_of_max_banks(all_banks: Vec<String>) -> i32 {
    let mut max = 0;
    for bank in all_banks {
        let max_item = nah_id_brute_force_it(&bank);
        max += max_item;
    }
    max
}


mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use crate::utils::get_file_contents;

    // #[test]
    // fn test_1_max_two_of_bank() {
    //     let sample1 = &"987654321111111".to_string();
    //     let result = max_two_of_bank(sample1);
    //     assert_eq!(result, (9, 8));
    // }

    // #[test]
    // fn test_2_max_two_of_bank() {
    //     let sample_2 = &"811111111111119".to_string();
    //     let result = max_two_of_bank(sample_2);
    //     assert_eq!(result, (8, 9));
    // }

    // #[test]
    // fn test_3_max_two_of_bank() {
    //     let sample_3 = &"234234234234278".to_string();
    //     let result = max_two_of_bank(sample_3);
    //     assert_eq!(result, (7, 8));
    // }

    // #[test]
    // fn test_4_max_two_of_bank() {
    //     let sample_4 = &"818181911112111".to_string();
    //     let result = max_two_of_bank(sample_4);
    //     assert_eq!(result, (9, 2));
    // }

    #[test]
    fn test_1_nah_id_brute_force_it() {
        let sample_1 = &"987654321111111".to_string();
        let result = nah_id_brute_force_it(sample_1);
        assert_eq!(result, 98);
    }

    #[test]
    fn test_2_nah_id_brute_force_it() {
        let sample_2 = &"811111111111119".to_string();
        let result = nah_id_brute_force_it(sample_2);
        assert_eq!(result, 89);
    }

    #[test]
    fn test_3_nah_id_brute_force_it() {
        let sample_3 = &"234234234234278".to_string();
        let result = nah_id_brute_force_it(sample_3);
        assert_eq!(result, 78);
    }

    #[test]
    fn test_4_nah_id_brute_force_it() {
        let sample_4 = &"818181911112111".to_string();
        let result = nah_id_brute_force_it(sample_4);
        assert_eq!(result, 92);
    }

    #[test]
    fn test_full_sample() {
        let full_sample = "987654321111111
                                811111111111119
                                234234234234278
                                818181911112111";
        let full_vec: Vec<String> = full_sample.lines().map(|f| f.trim().to_string()).collect();
        let result = find_sum_of_max_banks(full_vec);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_acutal_data() {
        let actual_data = get_file_contents("attachment_data_d3.md");
        let full_vec: Vec<String> = actual_data.lines().map(|f| f.trim().to_string()).collect();
        let result = find_sum_of_max_banks(full_vec);
        assert_eq!(result, 17_613);
    }
}