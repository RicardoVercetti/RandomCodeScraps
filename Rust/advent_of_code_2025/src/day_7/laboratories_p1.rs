
// walk through line by line
// let r be the row and c be column position
// if (rₙ, cₘ) is '.', and (rₙ₋₁, cₘ) is '|' or 'S',  (rₙ, cₘ) also becomes '|'
// if (rₙ, cₘ) is '^', and (rₙ₋₁, cₘ) is '|',  (rₙ, cₘ₋₁) and (rₙ, cₘ₊₁) becomes '|'
// do this from second row till the last row
// any '^' that has '|' above it is a count

#[allow(dead_code)]
fn create_stream(v: &Vec<String>) -> Vec<Vec<char>> {
    let mut result_stream:Vec<Vec<char>> = v.clone().iter().map(|s| s.chars().collect::<Vec<char>>()).collect();
    let max_row = result_stream.len();
    let max_col = result_stream[0].len();

    for row in 1..max_row {
        // go by each column starting from row 1
        let (first_split, second_split) = result_stream.split_at_mut(row);
        let all_items_in_row = &mut second_split[0];
        for col in 0..max_col {
            
            let last_item = &first_split[row-1][col];
            
            if all_items_in_row[col] == '.' && (last_item == &'|' || last_item == &'S') {
                // set this item to pipe
                all_items_in_row[col] = '|';
                // println!("set item");
            } else if all_items_in_row[col] == '^' && last_item == &'|' {
                // the before and after items change into pipes
                all_items_in_row[col-1] = '|';
                all_items_in_row[col+1] = '|';
            }
        }
    }

    // println!("modified:");
    // for i in result_stream {
    //     println!("{}", i.iter().collect::<String>());
    // }
    result_stream
}

#[allow(dead_code)]
fn find_beam_split_count(beam_of_streams: &Vec<Vec<char>>) -> i32 {
    // for each, if current is '^' and the one above is '|', its a count
    let mut count = 0;
    
    for (i, row) in beam_of_streams.iter().enumerate() {
        for (j, column_item) in row.iter().enumerate() {
            if i > 1 && column_item == &'^' && &beam_of_streams[i-1][j] == &'|' {
                count += 1;
            } 
        }
    }
    count
}

mod tests {
    #[allow(unused_imports)]
    use crate::utils::get_file_contents;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn sample_test() {
        let sample = ".......S.......
                        ...............
                        .......^.......
                        ...............
                        ......^.^......
                        ...............
                        .....^.^.^.....
                        ...............
                        ....^.^...^....
                        ...............
                        ...^.^...^.^...
                        ...............
                        ..^...^.....^..
                        ...............
                        .^.^.^.^.^...^.
                        ...............";
        let sample_data: Vec<String> = sample.lines().map(|l| l.trim().to_string()).collect();
        let streamed_out: Vec<Vec<char>> =  create_stream(&sample_data);
        let count = find_beam_split_count(&streamed_out);
        assert_eq!(count, 21);
    }

    #[test]
    fn test_acutal_data() {
        let sample = get_file_contents("attachment_data_d7.md");
        let sample_data: Vec<String> = sample.lines().map(|l| l.trim().to_string()).collect();
        let streamed_out: Vec<Vec<char>> =  create_stream(&sample_data);
        let count = find_beam_split_count(&streamed_out);
        assert_eq!(count, 1681);
    }
}