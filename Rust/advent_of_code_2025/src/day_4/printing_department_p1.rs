// can access if there are < 4 rolls of paper in the 8 adjacent positions
// in the example there are 13 rolls of paper that can be accessed by a forklift

// let i & j be the position if the item by row and column) 
// any item is accessible if atleast 3 of the positions of 
// (i-1)(j-1), (i-0)(j-1), (i+1)(j-1)
// (i-1)(j-0),           , (i+1)(j-0)
// (i-1)(j+1), (i-0)(j+1), (i+1)(j+1)
// are '.'
// for edges, the vales might not exist.
// therefore make sure that evaluation if 
//          -  (i+-x) does not go above or below the range of row ids
//          -  and (j+-x) does not go above or below colum length 

#[allow(dead_code)]
pub fn find_accessible_roll_count(accessibles: &Vec<Vec<char>>) -> i32 {
    let mut x_count = 0;
    for x in accessibles {
        for y in x {
            if *y == 'x' {
                x_count += 1;
            }
        }
    }
    x_count
}

#[allow(dead_code)]
pub fn replacer(full_vec: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    // loop through each element, get x, y, and the char, if its @, go ahead and see if its accessible
    let mut return_vec: Vec<Vec<char>> = Vec::new();

    for (x_pos, x_item) in full_vec.iter().enumerate() {
        let mut inner_vec: Vec<char> = Vec::new();
        for (y_pos, y_item) in x_item.iter().enumerate() {
            if y_item == &'@' && is_accessible(full_vec, x_pos, y_pos) {
                inner_vec.push('x');
            } else {
                inner_vec.push(*y_item);
            }
        }
        return_vec.push(inner_vec);
    }

    return_vec
}

fn is_accessible(vec: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let max_x = vec[0].len() - 1;       // always use upper bound inclusive looping
    let max_y = vec.len() - 1;

    let mut accessiblility_count = 0;

    // 1
    let pos_x_1: isize = x as isize - 1;
    let pos_y_1: isize = y as isize - 1;
    if inner_cond(pos_x_1, pos_y_1, max_x, max_y, &vec) {
        accessiblility_count += 1;
    }

    // 2
    let pos_x_2: isize = x as isize - 0;
    let pos_y_2: isize = y as isize - 1;
    if inner_cond(pos_x_2, pos_y_2, max_x, max_y, &vec) {
        accessiblility_count += 1;
    }

    // 3
    let pos_x_3: isize = x as isize + 1;
    let pos_y_3: isize = y as isize - 1;
    if inner_cond(pos_x_3, pos_y_3, max_x, max_y, &vec) {
        accessiblility_count += 1;
    }

    // 4
    let pos_x_4: isize = x as isize - 1;
    let pos_y_4: isize = y as isize - 0;
    if inner_cond(pos_x_4, pos_y_4, max_x, max_y, &vec) {
        accessiblility_count += 1;
    }

    // 5 is the current element

    // 6
    let pos_x_6: isize = x as isize + 1;
    let pos_y_6: isize = y as isize - 0;
    if inner_cond(pos_x_6, pos_y_6, max_x, max_y, &vec) {
        accessiblility_count += 1;
    }

    // 7
    let pos_x_7: isize = x as isize - 1;
    let pos_y_7: isize = y as isize + 1;
    if inner_cond(pos_x_7, pos_y_7, max_x, max_y, &vec) {
        accessiblility_count += 1;
    }

    // 8
    let pos_x_8: isize = x as isize - 0;
    let pos_y_8: isize = y as isize + 1;
    if inner_cond(pos_x_8, pos_y_8, max_x, max_y, &vec) {
        accessiblility_count += 1;
    }

    let pos_x_9: isize = x as isize + 1;
    let pos_y_9: isize = y as isize + 1;
    if inner_cond(pos_x_9, pos_y_9, max_x, max_y, &vec) {
        accessiblility_count += 1;
    }

    // println!("count at end: {}", accessiblility_count);

    accessiblility_count >= 5
}

fn inner_cond(x_pos: isize, y_pos: isize, x_max: usize, y_max: usize, vec: &Vec<Vec<char>>) -> bool {
    if x_pos < 0 || y_pos < 0 || x_pos as usize > x_max || y_pos as usize > y_max || vec[x_pos as usize][y_pos as usize] == '.' {
        return true;
    }
    false
}

mod tests {
    #[allow(unused_imports)]
    use crate::utils::get_file_contents;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn sample_test_1() {
        let sample = "..@@.@@@@.
                            @@@.@.@.@@
                            @@@@@.@.@@
                            @.@@@@..@.
                            @@.@@@@.@@
                            .@@@@@@@.@
                            .@.@.@.@@@
                            @.@@@.@@@@
                            .@@@@@@@@.
                            @.@.@@@.@.";
        let in_vec: Vec<Vec<char>> = sample.lines().map(|f| f.trim().chars().collect::<Vec<char>>()).collect();
        let replaced = replacer(&in_vec);
        let count = find_accessible_roll_count(&replaced);
        assert_eq!(count, 13);
    }

    #[test]
    fn test_actual_data() {
        let content = get_file_contents("attachment_data_d4.md");
        let vecs = content.lines().map(|l| l.trim().chars().collect::<Vec<char>>()).collect();
        let replaced = replacer(&vecs);
        let count = find_accessible_roll_count(&replaced);
        assert_eq!(count, 1395);
    }
}