use rand::Rng;
use std::io;

fn main() {
    println!("initiated...");
    //guessing_game();
    find_fib();
}


// Begenner tasks

// hello
#[allow(unused)]
fn greet(name:&str) {
    println!("Hell0, {name}");
}

// temperture converter(Fahrenheit to Celsius -> 32F-32 * 5/9)
#[allow(unused)]
fn convert_temperature(fhah: i32) -> f32 {
    (fhah as f32 - 32 as f32) * (5 as f32 / 9 as f32)
}

// guessing game
#[allow(unused)]
fn guessing_game() {
    println!("Welcome to the guessing game");
    let mut rng = rand::thread_rng();
    
    let n1: i32 = rng.gen_range(1..=100);
    
    println!("Generated num: {n1}");
    
    let mut is_done = false;
    
    while !is_done { 
        println!("Guess a number between 1 and 100(incusive)");
        let mut guess = String::new();    
           
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let num: i32 = match guess.trim()
                        .parse() {
                          Ok(num) => num,
                          Err(_) => {
                              println!("Please enter a valid number!");
                              continue;
                          }
                        };
                        
        if num == n1 {
            println!("You guessed right!");
            is_done = true;
        } else {
            println!("Try again!");
        }
    
    }
    println!("Thanks for playing...")
}

// Fibonnacci sequence - recursive and iterative version
fn find_fib() {
    println!("Find the fibbonacci number...");
    // 0 1 1 2 3
    
    let n: i64 = {
        let mut inp = String::new();
        io::stdin().read_line(&mut inp)
            .expect("Error taking input");
        inp.trim().parse().expect("Invalid input, have to be an integer!")
    };
    
    //let res = recursive_fib(n);
    let res = iterative_fib(n);
    println!("Result: {res}");
}

#[allow(unused)]
fn recursive_fib(n: i64) -> i64 {
    if n == 1 { 0 } else if n == 2 { 1 } else {
         recursive_fib(n-1) + recursive_fib(n-2)
    }
}

fn iterative_fib(n: i64) -> i64 {
    let mut val = 0;
    let mut loo: i64 = 0;
    let mut v1 = 0;
    let mut _v2 = 0;
    
    while loo <= n {
        if loo == 2 {
            val+=1;
        } else if loo == 2 {
            v1 = val;    // v1 = 1, val = 1
            val = val + v1;
        } else {
            _v2 = v1;
            v1 = val;
            val = _v2 + v1;
        }
        
        loo+=1;
    }
    val
}


