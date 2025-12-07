// now, instead of two, havfta find 12 batteries to turn on on each bank, and thats gotta be max

#[allow(dead_code)]
pub fn max_twelve_in_a_bank(bank: &String) -> i64 {         // this is too bad
    // the brute force would be having a 12 nested loop
    let mut current_max: i64 = 0;

    for (i, item_i) in bank.chars().enumerate() {
        for (j, item_j) in bank.chars().enumerate() {
            for (k, item_k) in bank.chars().enumerate() {
                for (l, item_l) in bank.chars().enumerate() {
                    for (m, item_m) in bank.chars().enumerate() {
                        for (n, item_n) in bank.chars().enumerate() {
                            for (o, item_o) in bank.chars().enumerate() {
                                for (p, item_p) in bank.chars().enumerate() {
                                    for (q, item_q) in bank.chars().enumerate() {
                                        for (r, item_r) in bank.chars().enumerate() {
                                            for (s, item_s) in bank.chars().enumerate() {
                                                for (t, item_t) in bank.chars().enumerate() {
                                                    // update every now and then
                                                    // print!("\rUpdate on i, m, p: {}, {}, {}", i, m, p);
                                                    if t > s && s > r && r > q && q > p && p > o && o > n && n > m && m > l && l > k && k > j && j > i {
                                                        let max_candidate: i64 = format!("{}{}{}{}{}{}{}{}{}{}{}{}", item_i, item_j, item_k, item_l, item_m, item_n, item_o, item_p, item_q, item_r, item_s, item_t).parse().unwrap();
                                                        if max_candidate > current_max {
                                                            current_max = max_candidate;
                                                            println!("condition satisfied for i: {}, cand: {}", i, max_candidate);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    current_max
}

// alternate brute way :)
// 234_234_234_234_278 → 12 chars  4 _ 3 4 _ 2 3 4 _ 2 3 4 _ 2 7 8
// n is the number of charecters required, 
// pick the last first charecter to nth from the last and find the largest, 
// l1 be the largest and p1 is the position of that l1 
// now look for l2 after p1 and n-1 postition of whole charecter, the largest is l2, position is p2
// cycle continues until all of n is exhausted
#[allow(dead_code)]
pub fn find_max_the_smart_way(s: &String) -> i64 {
    // lets say n is 12 for now
    let mut n = 12;
    let mut top_range = 0;
    let mut bottom_range = s.len() - n; // 12 - 1 = 11 cuz indexing is 11 for the 12 th charecter from last(or use range inclusive like ..=)
    // println!("bottom range: {}", bottom_range);

    let mut collected: Vec<String> = Vec::new();

    while n > 0 {
        let range: &str = &s[top_range..=bottom_range];
        // println!("{} range: {}", n, range);

        // find max in range(as leftmost as possible) and the position of that max in range
        let mut max = 0;
        let mut max_pos: usize = 0;
        for (i, item) in range.chars().enumerate() {
            let num = item.to_digit(10).unwrap();
            // println!("each num : {}", num);
            // println!("each char: {}", item);
            if num > max {
                max = num;
                max_pos = i;
            }
        }

        collected.push(max.to_string());
        top_range = top_range + max_pos + 1;    // exclusive of the selected element's position
        bottom_range += 1;
        n -=1;
    }

    // println!("collected: {:?}", collected);
    collected.join("").parse::<i64>().unwrap()
}

#[allow(dead_code)]
pub fn find_sum_of_max_banks_smart_way(all_banks: Vec<String>) -> i64 {
    let mut max: i64 = 0;
    for bank in all_banks {
        max += find_max_the_smart_way(&bank);
    }
    max
}


mod tests {
    #[allow(unused_imports)]
    use crate::utils::get_file_contents;

    #[allow(unused_imports)]
    use super::*;

    // #[test]
    // fn sample_1_max_twelve_in_bank() {          // WARN: this runs forever :)
    //     let sample_1 = &"987654321111111".to_string();
    //     let result = max_twelve_in_a_bank(sample_1);
    //     assert_eq!(result, 987654321111);
    // }

    #[test]
    fn sample_1_max_smart_way() {
        let sample_1 = &"987654321111111".to_string();
        let result = find_max_the_smart_way(sample_1);
        assert_eq!(result, 987654321111 as i64);
    }

    #[test]
    fn sample_2_max_smart_way() {
        let sample_2 = &"811111111111119".to_string();
        let result = find_max_the_smart_way(sample_2);
        assert_eq!(result, 811111111119 as i64);
    }

    #[test]
    fn sample_3_max_smart_way() {
        let sample_3 = &"234234234234278".to_string();
        let result = find_max_the_smart_way(sample_3);
        assert_eq!(result, 434234234278 as i64);
    }

    #[test]
    fn sample_4_max_smart_way() {
        let sample_4 = &"818181911112111".to_string();
        let result = find_max_the_smart_way(sample_4);
        assert_eq!(result, 888911112111 as i64);
    }

    #[test]
    fn test_all_sample() {
        let samples = "987654321111111
                            811111111111119
                            234234234234278
                            818181911112111";
        let all_banks: Vec<String> = samples.lines().map(|f| f.trim().to_string()).collect();
        let result = find_sum_of_max_banks_smart_way(all_banks);
        assert_eq!(result, 3121910778619 as i64);
    }

    #[test]
    fn test_real_data() {
        let content = get_file_contents("attachment_data_d3.md");
        let all_banks: Vec<String> = content.lines().map(|f| f.trim().to_string()).collect();
        let result = find_sum_of_max_banks_smart_way(all_banks);
        assert_eq!(result, 175304218462560 as i64);
    }
}