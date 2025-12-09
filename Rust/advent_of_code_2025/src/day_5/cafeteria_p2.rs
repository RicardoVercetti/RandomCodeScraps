use std::collections::BTreeSet;

#[allow(dead_code)]
pub fn find_all_fresh_in_ranges(ranges: &Vec<(i64, i64)>) -> BTreeSet<i64> {
    let mut set: BTreeSet<i64> = BTreeSet::new();

    for (left, right) in ranges {
        println!("\rchecking range: {}:{}", left, right);
        for i in *left..=*right {
            set.insert(i);
        }
    }
    set
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