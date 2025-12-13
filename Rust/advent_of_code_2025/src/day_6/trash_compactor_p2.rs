
// part 2 approach:
// if +, right pad it by max length of all items in row with 0
// if *, left pad it by max length of all items in row with 1
// then use the same function to calculate on the fly

// same as in trash_compactor_p1::separator, just that instead of returning Vec<Vec<i32>>, this returns Vec<Vec<String>>
#[allow(dead_code)]
pub fn separator(inp: &Vec<String>) -> (Vec<Vec<String>>, Vec<String>) {
    let (all_numbers, all_operators) = inp.split_at(inp.len()-1);
    // let mut array_of_numbers: Vec<Vec<i32>> = Vec::new();
    let all_operator_array: Vec<String> = (&all_operators[0]).split_whitespace().map(|f| f.to_string()).collect();
    let mut all_array_of_number_arrays: Vec<Vec<String>> = Vec::new();

    let no_of_arr = all_numbers.len();
    

    for i in 0..no_of_arr {
        let arr_item: Vec<String> = all_numbers[i].split_whitespace().map(|f| f.trim().parse::<String>().unwrap()).collect();
        all_array_of_number_arrays.push(arr_item);
    }

    let no_of_inner_arr = all_array_of_number_arrays.len();

    // println!("all_operator: {:?}", all_operator_array);
    // println!("all number: {:?}", all_array_of_number_arrays);

    let mut tup_arr: Vec<Vec<String>> = Vec::new();

    for (i, oper) in all_operator_array.iter().enumerate() {
        let mut one_vec = Vec::new();
        for j in 0..no_of_inner_arr {
            let max_len = max_length_of_row(&all_array_of_number_arrays[j]);
            if oper == "+" {
                one_vec.push(right_pad_zero(&all_array_of_number_arrays[j][i], max_len));
            } else if oper == "*" {
                one_vec.push(left_pad_one(&all_array_of_number_arrays[j][i], max_len));
            } else {
                unimplemented!("impossible condition");
            }
        }
        tup_arr.push(one_vec);
    }
    (tup_arr, all_operator_array)
}

fn max_length_of_row(row: &Vec<String>) -> usize{
    let mut max = 0;
    for i in row {
        if i.len() > max {
            max = i.len();
        }
    }
    max
}

fn right_pad_zero(s: &String, max_len: usize) -> String {
    format!("{:g<size$}", s, size = max_len)
}

fn left_pad_one(s: &String, max_len: usize) -> String {
    format!("{:g>size$}", s, size = max_len)
}

/// column wise addition of the output of trash_compactor_p2::separator
/// input: ["123", "g45", "gg6"] output: colums wise of each row as ["1", "24", "356"]
///        ["328", "64g", "98g"]                                    ["369", "248", "8"]     
///        ["g51", "387", "215"]                                    ["32", "581", "175"]         
///        ["64g", "23g", "314"]                                    ["623", "431", "4"]         
#[allow(dead_code)]
pub fn get_column_wise(inp: &(Vec<Vec<String>>, Vec<String>)) -> Vec<Vec<String>> {
    let mut res = Vec::new();
    for row in &inp.0 {
        let max_len = max_length_of_row(&row);
        let mut collec = Vec::new();
        for i in 0..max_len {
            let mut per_item = Vec::new();
            for item in row {
                let sstr = item.chars().nth(i).unwrap();
                if sstr != 'g' {
                    per_item.push((sstr as i64 - '0' as i64).to_string());
                }
            }
            collec.push(per_item.join(""));
        }
        res.push(collec);
    }
    res
}

#[allow(dead_code)]
pub fn calculate(nums: &Vec<Vec<String>>, opers: &Vec<String>) -> Vec<i64> {
    let mut sum_prd_of_each_row = Vec::new();
    for i in 0..opers.len() {
        let row = &nums[i];
        if opers[i] == "*" {
            sum_prd_of_each_row.push(loop_n_mult(&row));
        } else if opers[i] == "+" {
            sum_prd_of_each_row.push(loop_n_sum(&row));
        } else {
            unreachable!();
        }
    }
    sum_prd_of_each_row
}

fn loop_n_sum(row: &Vec<String>) -> i64 {
    let mut count: i64 = 0;
    for item in row {
        if item.is_empty() {
            continue;
        }
        count += item.parse::<i64>().unwrap();
    }
    count
}

fn loop_n_mult(row: &Vec<String>) -> i64 {
    let mut count: i64 = 1;
    for item in row {
        if item.is_empty() {
            continue;
        }
        count *= item.parse::<i64>().unwrap();
    }
    count
}

#[allow(dead_code)]
pub fn grand_total(arr: &Vec<i64>) -> i128 {
    let mut total: i128 = 0;
    for i in arr {
        total += *i as i128;
    }
    total
}

mod tests {
    #[allow(unused_imports)]
    use crate::utils::get_file_contents;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "123 328  51 64 
                            45 64  387 23 
                            6 98  215 314
                            *   +   *   +  ";
        let lines: Vec<String> = sample.lines().map(|l| l.to_string()).collect();
        let sep = separator(&lines);
        let col_wise = get_column_wise(&sep);

        let res_li = calculate(&col_wise, &sep.1);
        let res = grand_total(&res_li);
        assert_eq!(res, 3263827 as i128);
    }

    #[test]
    fn test_acutal_data() {
        let sample = get_file_contents("attachment_data_d6.md");
        let lines: Vec<String> = sample.lines().map(|l| l.to_string()).collect();
        let sep = separator(&lines);
        let col_wise = get_column_wise(&sep);

        let res_li = calculate(&col_wise, &sep.1);
        let res = grand_total(&res_li);
        assert_eq!(res, 3263827 as i128);       // the output of this is not the right answer
    }
}