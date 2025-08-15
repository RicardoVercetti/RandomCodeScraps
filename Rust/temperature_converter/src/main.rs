use std::io;

// -- a console program that --
// 1. lets the user choose conversion type
// 2. read the temperature from the user
// 3. prints the converted value
// 4. Loops until user enters the value 'q'

fn c_to_f(c: f32) -> f32 {
    c * 9.0/5.0 + 32.0
}

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn main() {
    println!("-- Welcome to temperature converter --");
    
    loop {
    println!("\n=====================================");
    println!("Press 1 for Celcius to Fahrenheit");
    println!("Press 2 for Fahrenheit to Celcius");
    println!("q: quit");
    println!("=====================================\n");
    println!("Enter the choice:");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    
    
    let choice = choice.trim();
    
    if choice == "q" {
        println!("Goodbye...!");
        return;
    }
    
    //println!("You entered: {}", choice);
    
    // read input
    let mut input = String::new();
    println!("Enter input: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let temp: f32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };
    
    
    // print converted
    match choice {
        "1" => println!("{} deg C = {:.2} deg F", temp, c_to_f(temp)),
        "2" => println!("{} deg F - {:.2} deg C", temp, f_to_c(temp)),
        _ => println!("Invalid choice!"),
    };
    
    
    }
    
}
