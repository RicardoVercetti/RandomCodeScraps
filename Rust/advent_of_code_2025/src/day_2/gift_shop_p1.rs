// leading zeros are not ids → 0101 isnt id at all while 101 can be ignored
// invalid IDs are the ones that have repeating numbers. Eg: 998-1012 has one invalid ID, 1010
// give back the sum of all invalid IDs

// soln idea:
// every atomic range exist in tens  1-9, 10-99, 100-999
// there could be a little of one or more of atomic ranges in a standard range in the input
// if an atomic range is odd, (ie: 10-99 is even, and 100-999 is odd) no repeating invalid IDs can be found there.
// if the atomic range is even, take as string, split to two half and compare with the other half
// do a for loop of every doable atomic range like this

#[allow(dead_code)]
fn find_automic_range(s: &String) -> Vec<String> {
    let mut return_str = Vec::new();
    let li: Vec<&str> = s.split("-").map(|f| f.trim()).collect();
    let lhs = li.get(0).unwrap();
    let rhs = li.get(1).unwrap();

    // println!("lhs|rhs: {}|{}", lhs, rhs);

    // case:1 - lhs and rhs are in same range
    if lhs.len() == rhs.len() {
        return_str.push(format!("{}-{}", lhs, rhs));
    } else {// case: 2 - lhs and rhs are in different range
        // go by tens
        // 12-100 = 12-99, 100: 1          500-13_000 = 500-999, 1000-9999, 10_000-13_000: 2
        let num_diff = rhs.len() - lhs.len(); // here the num difference will always be 1 or more than 1
        
        let mut last_dig: String = "".to_string();      // the init is done just cuz of formality
        for i in 0..=num_diff {
            // if i == 0
            if i == 0 {
                let l = lhs.to_string();
                let r = "9".repeat(lhs.len());
                last_dig = r.clone();
                return_str.push(format!("{}-{}", l, r));
            }

            // inbetweens
            else if i > 0 && i != num_diff {
                let l = format!("1{}", "0".repeat(last_dig.len()));
                let r = "9".repeat(last_dig.len() + 1);
                last_dig = r.clone();
                return_str.push(format!("{}-{}", l, r));
            }

            // if i == num_diff
            else if i==num_diff {
                let l = format!("1{}", "0".repeat(last_dig.len()));
                let r = rhs.to_string();
                // no need to update last_dig anymore
                return_str.push(format!("{}-{}", l, r));
            }
        }
        
    }

    return_str
}

#[allow(dead_code)]
fn find_repeating_in_range(vecs: Vec<String>) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    for item in vecs {

        let items: Vec<&str> = item.split("-").collect();
        // println!("items: {:?}", items);
        let left: i64 = items.get(0).unwrap().parse::<i64>().unwrap();       // converting to integer removes the leading zeros like 0101
        let right: i64 = items.get(1).unwrap().parse::<i64>().unwrap();

        // item has lhs and rhs, loop through it from start to end inclusive
        let len_str = left.to_string().len();

        if len_str % 2 != 0 {
            continue;       // if odd, we can ignore
        }
        let half = len_str / 2;  // the length is going to be same for all the elements in the range

        for i in left..=right {
            let nr_str = i.to_string();
            // println!("nr_str: '{}'", nr_str);
            // println!("half: {}", half);
            // println!("len: {}", len_str);
            
            if &nr_str[0..half] == &nr_str[half..nr_str.len()] {      // 4(1234) → [0..2](1,2), [2..4](3, 4), 2(11) → [0..1](1), [1..2]()
                found.push(nr_str.to_string());
            }
        }
        
    }
    found
}

#[allow(dead_code)]
fn the_solution(st: String) -> i64 {
    let split_of_ranges:Vec<String> = st.split(",").map(|f| f.trim().to_string()).collect();
    let mut all_atomic_ranges: Vec<String> = Vec::new();
    for items in split_of_ranges {
        let mut out = find_automic_range(&items);
        all_atomic_ranges.append(&mut out);
    }
    
    // println!("all automic ranges: {:?}", all_atomic_ranges);
    let result = find_repeating_in_range(all_atomic_ranges);
    
    // println!("repeating: {:?}", result);
    let mut sum: i64= 0;
    for i in result {
        sum = sum + i.parse::<i64>().unwrap();
    }
    sum
}

mod tests {
    #[allow(unused_imports)]
    use crate::utils::get_file_contents;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn sample_test() {
        let sample = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        
        let result = the_solution(sample.to_string());

        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_actual_data() {
        let data = get_file_contents(&"attachment_data_d2.md");
        let result = the_solution(data);
        assert_eq!(result, 29818212493);
    }
}