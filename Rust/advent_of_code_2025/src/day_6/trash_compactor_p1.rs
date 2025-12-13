#[allow(dead_code)]
fn sum_it_up(arr: &Vec<i64>) -> i64 {
    let mut sum: i64 = 0;
    for i in arr {
        sum += i;
    }
    sum
}

#[allow(dead_code)]
/// for an input of 
/// [["123", "45", "6"],            ["*", "+", "*", "+"]
///  ["328", "64", "98"],
///  ["51", "387", "215"],
///  ["64", "23", "314"]]
/// 
/// the ouput would be(sum of each row)
/// [33210, 490, 4243455, 401]
pub fn calculate(amalgam: &(Vec<Vec<i32>>, Vec<String>)) -> Vec<i64> {
    let (arrays, iterables) = amalgam;
    let len: usize = iterables.len();
    let mut result: Vec<i64> = Vec::new();
    for i in 0..len {
        if iterables[i] == "+" {
            let mut one_sum: i64 = 0;
            for item in 0..arrays[i].len() {
                one_sum += arrays[i][item] as i64;
            }
            result.push(one_sum);
        } else if iterables[i] == "*" {
            let mut one_mult: i64 = 1;
            for item in 0..arrays[i].len() {
                one_mult *= arrays[i][item] as i64;
            }
            result.push(one_mult);
        } else {
            unreachable!("this is an unreachable condition: {}", iterables[i]);
        }
    }
    result
}

#[allow(dead_code)]
/// input : &Vec<String>
/// ["123 328  51 64",
/// "45 64  387 23",
/// "6 98  215 314",
/// "*   +   *   +"]
///  
/// output: (Vec<Vec<i32>>, Vec<String>)
/// [["123", "45", "6"],    ["*", "+", "*", "+"]
/// ["328", "64", "98"],
/// ["51", "387", "215"],
/// ["64", "23", "314"]]    
pub fn separator(inp: &Vec<String>) -> (Vec<Vec<i32>>, Vec<String>) {
    let (all_numbers, all_operators) = inp.split_at(inp.len()-1);
    // let mut array_of_numbers: Vec<Vec<i32>> = Vec::new();
    let all_operator_array: Vec<String> = (&all_operators[0]).split_whitespace().map(|f| f.to_string()).collect();
    let mut all_array_of_number_arrays: Vec<Vec<i32>> = Vec::new();

    let no_of_arr = all_numbers.len();
    

    for i in 0..no_of_arr {
        let arr_item: Vec<i32> = all_numbers[i].split_whitespace().map(|f| f.trim().parse::<i32>().unwrap()).collect();
        all_array_of_number_arrays.push(arr_item);
    }

    let no_of_inner_arr = all_array_of_number_arrays.len();

    // println!("all_operator: {:?}", all_operator_array);
    // println!("all number: {:?}", all_array_of_number_arrays);

    let mut tup_arr: Vec<Vec<i32>> = Vec::new();

    for i in 0..all_operator_array.len() {
        let mut one_vec = Vec::new();
        for j in 0..no_of_inner_arr {
            one_vec.push(all_array_of_number_arrays[j][i]);
        }
        tup_arr.push(one_vec);
    }
    (tup_arr, all_operator_array)
}

#[allow(dead_code)]
fn find_sum_of_all_rows(inp: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in inp {
        sum += i;
    }
    sum
}

#[allow(dead_code)]
fn compute_per_each_row(all_rows: &Vec<(i32, i32, i32, String)>) -> Vec<i32> {
    let mut ret_vec: Vec<i32> = Vec::new();
    for (item_1,item_2, item_3, op) in all_rows {
        if op == "+" {
            ret_vec.push(*item_1 + item_2 + item_3);
        } else if op == "*" {
            ret_vec.push(*item_1 * item_2 * item_3);
        } else {
            unreachable!("invalid condition: {}", op);
        }
    }
    ret_vec
}

#[allow(dead_code)]
fn convert_to_tuples(inp: &Vec<Vec<String>>) -> Vec<(i32, i32, i32, String)> {
    let mut per_item_vec: Vec<(i32, i32, i32, String)> = Vec::new();

    for i in 0..inp.len() {
        let item_a = &inp[0][i];
        let item_b = &inp[1][i];
        let item_c = &inp[2][i];
        let oper = &inp[3][i];

        let tup: (i32, i32, i32, String) = (item_a.parse().unwrap(), item_b.parse().unwrap(), item_c.parse().unwrap(), oper.trim().to_string());
        per_item_vec.push(tup);
    }

    per_item_vec
}

mod tests {
    #[allow(unused_imports)]
    use crate::utils::get_file_contents;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn sample_test() {
        let sample = "123 328  51 64 
                        45 64  387 23 
                        6 98  215 314
                        *   +   *   + ";
        let lines: Vec<String> = sample.lines().map(|l| l.trim().to_string()).collect();
        let res: (Vec<Vec<i32>>, Vec<String>) = separator(&lines);
        let calc = calculate(&res);

        let sum = sum_it_up(&calc);
        assert_eq!(sum, 4277556);
    }

    #[test]
    fn test_actual_data() {
        let sample = get_file_contents("attachment_data_d6.md");
        let lines: Vec<String> = sample.lines().map(|l| l.trim().to_string()).collect();
        let res: (Vec<Vec<i32>>, Vec<String>) = separator(&lines);
        let calc = calculate(&res);

        let sum = sum_it_up(&calc);
        assert_eq!(sum, 4719804927602);
    }
}