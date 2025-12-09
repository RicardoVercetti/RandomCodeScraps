
#[allow(dead_code)]
fn find_spoiled_items(all_items: &Vec<i64>, all_ranges: &Vec<(i64, i64)>) -> Vec<i64> {
    let mut spoiled_items = Vec::new();
    for item in all_items {
        if !is_in_any_range(item, all_ranges) {
            spoiled_items.push(item.clone());
        }
    }

    spoiled_items
}

#[allow(dead_code)]
fn find_fresh_items(all_items: &Vec<i64>, all_ranges: &Vec<(i64, i64)>) -> Vec<i64> {
    let mut fresh_items = Vec::new();
    for item in all_items {
        if is_in_any_range(item, all_ranges) {
            fresh_items.push(item.clone());
        }
    }

    fresh_items
}

fn is_in_any_range(num: &i64, ranges: &Vec<(i64, i64)>) -> bool {
    ranges.iter().any(|f| f.0 <= *num && f.1 >= *num)
}

#[allow(dead_code)]
pub fn separate_data(s: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    // the ranges and the IDs are separated by a blank line
    let mut ranges: Vec<String> = Vec::new();
    let mut items: Vec<String> = Vec::new();

    for line in s.lines() {
        if line.contains("-") {
            ranges.push(line.trim().to_string());
        } else if line.trim().len() > 0 {
            items.push(line.trim().to_string());
        }
    }

    (convert_ranges(&ranges), convert_numbers(&items))
}

fn convert_ranges(ranges: &Vec<String>) -> Vec<(i64, i64)> {
    let mut converted_range: Vec<(i64, i64)> = Vec::new();
    for item in ranges {
        let split: Vec<&str> = item.split("-").collect();
        converted_range.push((split[0].parse::<i64>().unwrap() , split[1].parse::<i64>().unwrap()));
    }
    converted_range
}

fn convert_numbers(numbers: &Vec<String>) -> Vec<i64> {
    numbers.iter().map(|item| item.trim().parse::<i64>().unwrap()).collect()
}

mod tests {
    #[allow(unused_imports)]
    use crate::utils::get_file_contents;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_sample() {
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
        let (ranges, items) = separate_data(&sample);
        let fresh_items = find_fresh_items(&items, &ranges);
        assert_eq!(fresh_items.len(), 3);
    }

    #[test]
    fn test_actual_data() {
        let acutal_data = get_file_contents("attachment_data_d5.md");
        let (ranges, items) = separate_data(&acutal_data);
        let fresh_items = find_fresh_items(&items, &ranges);
        assert_eq!(fresh_items.len(), 525);
    }
}