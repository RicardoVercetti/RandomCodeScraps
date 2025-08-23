// Task 
// Take input from the user
// handle all errors/exceptions gracefully.

use std::io;

#[derive(Debug)]
enum ResponseTypes {
    Io(io::Error),
    Quit,
}

fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b != 0.0 {
        Ok(a/b)
    } else {
        Err("Division by Zero".to_string())
    }
}

fn get_input(s: &str) -> Result<f64, ResponseTypes> {
    loop {
        let mut var = String::new();
        println!("Enter {} :", s);
        io::stdin().read_line(&mut var).map_err(ResponseTypes::Io)?;
        let var =  match var.trim() {
            "q" => {
                return Err(ResponseTypes::Quit);
            },
            other => other,
        };
        
        let flt: f64 = match var.parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Please enter a valid floating point number");
                continue;
            },
        };
        
        
        return Ok(flt);
    }
    
}

fn main() {
    println!("-- safe divide --");
    println!("Press q to quit anytime");
    
    loop {
        
        let a = match get_input("First Number") {
            Ok(val) => val,
            Err(ResponseTypes::Quit) => {
                println!("Goodbye...");
                break;
            },
            Err(ResponseTypes::Io(e)) => {
                println!("Unable to read the stdin!");
                println!("Err : {}", e);
                break;
            }
        };
        
        let b = match get_input("Second Number") {
            Ok(val) => val,
            Err(ResponseTypes::Quit) => {
                println!("Goodbye...");
                break;
            },
            Err(_) => {
                println!("Unable to read the stdin!");
                break;
            },
        };
        
        
        match safe_divide(a, b) {
            Ok(data) => println!("Result: {}", data),
            Err(e) => println!("Err: {}", e), 
        }
        
        println!("If you're not satisfied with the result, you can try again!");
    }
    
}
