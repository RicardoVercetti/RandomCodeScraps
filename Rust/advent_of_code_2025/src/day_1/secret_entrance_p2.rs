#[allow(dead_code)]
#[derive(PartialEq, Eq)]
enum Operation {
    ADD,
    SUB
}

#[allow(dead_code)]
fn find_password_again(content: Vec<String>) -> i32 {
    // loop one by one, and count if there is a mod of 100 or an equal of 0 exists

    let mut count: i32 = 0;
    let mut start = 50;
    for i in content {
        
        let curr_no: i32;
        let opr: Operation;

        curr_no = i[1..].parse::<i32>().unwrap();
        
        if i.starts_with("R") {
            opr = Operation::ADD;
        
        } else if i.starts_with("L") {
            opr = Operation::SUB;

        } else {
            todo!("There is something wrong with your input...");
        }

        for _ in 0..curr_no {
            
            if opr == Operation::ADD {
                start = start + 1;
            } else {
                start = start - 1;
            }

            if start == 0 || start % 100 == 0 {
                count = count + 1;

            }
        }
    }

    // finally the password is the count
    // println!("count is: {}", count);
    count
}


mod tests {
    #[allow(unused_imports)]
    use crate::day_1::secret_entrance_p1::get_file_contents;
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn sample_test() {
        let sample_data_str = "L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82";

        let sample_data: Vec<String> = sample_data_str.lines().map(|f| f.trim().to_string()).collect();
        let result = find_password_again(sample_data);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_actual_data() {
        let sample_data = get_file_contents();
        let result = find_password_again(sample_data);
        assert_eq!(result, 6254);

    }
}