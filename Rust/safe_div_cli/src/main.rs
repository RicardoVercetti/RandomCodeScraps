// Task 
// Take input from the user
// handle all errors/exceptions gracefully.

use std::io;

fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b != 0.0 {
        Ok(a/b)
    } else {
        Err("Division by Zero".to_string())
    }
}

fn main() {
    println!("-- safe divide --");
    println!("Press q to quit anytime");
    
    'mainloop: loop {
        
        let a: f64 = 'a_loop: loop {
            println!("Enter the first number: ");
            let mut a = String::new();
            if let Err(e) = io::stdin().read_line(&mut a) {
                    println!("Input stream error occured: {}", e);
                    break 'mainloop;
            };
            
            // check if its 'q'
            let trimmed_a = match a.trim() {
                "q" => {
                    println!("Goodbye...");
                    break 'mainloop;
                },
                r => r,
            };
            
            let a: f64 = match trimmed_a.parse() {
                Ok(val) => val,
                Err(_) => {
                    println!("Please enter a valid floating point number");
                    continue 'a_loop;
                },
            };
            break 'a_loop a;
        };
        
        let b: f64 = 'b_loop: loop {
            println!("Enter the second number: ");
            let mut b = String::new();
            if let Err(_) = io::stdin().read_line(&mut b){
                println!("Unable to read input from console!");
                println!("Terminating application...");
                break 'mainloop;
            };
            
            // check if its 'q'
            let trimmed_b = match b.trim() {
                "q" => {
                    println!("Goodbye...");
                    break 'mainloop;
                },
                r => r,
            };
            let b: f64 = match trimmed_b.parse() {
                Ok(val) => val,
                Err(_) => {
                    println!("Please enter a valid floating point number");
                    continue 'b_loop;
                },
            };
            break 'b_loop b;
        };
        
        
        match safe_divide(a, b) {
            Ok(data) => println!("Result: {}", data),
            Err(e) => println!("Err: {}", e), 
        }
        
        println!("If you're not satisfied with the result, you can try again!");
    }
    
}
