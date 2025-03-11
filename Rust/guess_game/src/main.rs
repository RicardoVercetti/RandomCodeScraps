use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("Guess the number between 1 and 10!");
    // println!("The number is : ", secret_number);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if guess < secret_number {
            println!("Too low! Try again.");
        } else if guess > secret_number {
            println!("Too high! Try again.");
        } else {
            println!("🎉 Congratulations! You guessed the right number.");
            break;
        }
    }
}
