fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b != 0.0 {
        Ok(a/b)
    } else {
        Err("Division by Zero".to_string())
    }
}

fn main() {
    println!("Hello, world!");
    
    let a = 4.0;
    let b = 0.0;
    
    match safe_divide(a, b) {
        Ok(data) => println!("Result: {}", data),
        Err(e) => println!("Err: {}", e), 
    }
    
}
