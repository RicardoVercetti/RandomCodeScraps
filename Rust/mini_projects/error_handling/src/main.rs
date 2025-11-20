use std::fs;

#[allow(dead_code)]
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a/b)   
    }
}

#[allow(dead_code)]
fn result_type() {
    match divide(1.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    };
}

#[allow(dead_code)]
fn read_file() -> Result<String, std::io::Error> {
    let contents = fs::read_to_string("Hello.txt")?;
    Ok(contents)
}

#[allow(dead_code)]
fn q_mark_operator() {
    match read_file() {
        Ok(data) => println!("File contents : {}", data),
        Err(e) => println!("Error reading file: {}", e),
    }
}

fn find_char(s: &str, c: char) -> Option<usize> {
    s.find(c)
}

fn main() {
    println!("Hello, Universe...");
    
    //result_type();
    //q_mark_operator();
    match find_char("rustacean", 'm') {
        Some(pos) => println!("Found at {}", pos),
        None => println!("Not found"),
    }
}
