use super::printing_department_p1::{
    find_accessible_roll_count,
    replacer
};

// keep removing rolls and find maximum roles that can be removed

#[allow(dead_code)]
pub fn refresh_after_replaced(vec: &mut Vec<Vec<char>>) {
    for row in vec {
        for item in row {
            if *item == 'x' {
                *item = '.';
            }
        }
    }
}

#[allow(dead_code)]
fn loop_until_all_done(inp: Vec<Vec<char>>) -> i32 {
    let mut cln_inp = inp.clone();

    let mut total_removed = 0;
    let mut current_removed_not_zero = true;
    let mut _total_loops = 0;

    while current_removed_not_zero {
        let mut replaced = replacer(&cln_inp);
        let count = find_accessible_roll_count(&replaced);
        if count > 0 {
            total_removed += count;
        } else {
            current_removed_not_zero = false;
        }
        _total_loops += 1;
        refresh_after_replaced(&mut replaced);
        cln_inp = replaced;
    }

    total_removed
}

mod tests {
    #[allow(unused_imports)]
    use crate::utils::get_file_contents;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_sample() {
        let sample_string = "..@@.@@@@.
                                    @@@.@.@.@@
                                    @@@@@.@.@@
                                    @.@@@@..@.
                                    @@.@@@@.@@
                                    .@@@@@@@.@
                                    .@.@.@.@@@
                                    @.@@@.@@@@
                                    .@@@@@@@@.
                                    @.@.@@@.@.";
        let vecs: Vec<Vec<char>> = sample_string.lines().map(|l| l.trim().chars().collect::<Vec<char>>()).collect();
        let total_count = loop_until_all_done(vecs);
        assert_eq!(total_count, 43);
    }

    #[test]
    fn test_acutal_data() {
        let content = get_file_contents("attachment_data_d4.md");
        let vecs: Vec<Vec<char>> = content.lines().map(|l| l.trim().chars().collect::<Vec<char>>()).collect();
        let total_count = loop_until_all_done(vecs);
        assert_eq!(total_count, 8451);
    }
}