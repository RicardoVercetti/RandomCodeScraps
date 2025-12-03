use std::{fs::File, io::Read};

#[allow(dead_code)]
fn get_file_contents() -> Vec<String> {
    let mut file = File::open("attachment_data.md").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    // convert to vec
    let mut lines: Vec<String> = content.lines().map(|f| f.to_string()).collect();

    // strip out last line and first line
    lines.remove(0);
    lines.pop();
    lines
}

#[allow(dead_code)]
fn find_password(contents: Vec<String>) -> i32 {

    let mut num = 50;
    let mut rotation_count = 0;


    for i in contents {
        let curr_no: i32;
        if i.starts_with("L") {
            curr_no = i[1..].parse::<i32>().unwrap();
            num = num - curr_no;
        } else if i.starts_with("R") {
            curr_no = i[1..].parse::<i32>().unwrap();
            num = num + curr_no;
        } else {
            // this condition should never happen
            todo!("each rotation should start with either R or L. This is an invalid line: {}", i);
        }

        if num == 0 || num % 100 == 0 {
            // println!("condition satisfied at: {}", num);
            rotation_count = rotation_count + 1;
        }
    }

    // println!("done counting, count: {}", rotation_count);

    rotation_count
}


mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_sample() {
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
        // println!("content: {:?}", sample_data);

        let result = find_password(sample_data);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_actual_data() {
        let contents = get_file_contents();
        let result = find_password(contents);
        assert_eq!(result, 1074);
    }
}