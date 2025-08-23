// Task

// 1. Add rand as dependency
// 2. Create list/array of greetings(eg: Hello, Hi, Welcome, yo)
// 3. Pick one randomly and print it
// 4. Also include the current time stamp using chrono



use rand::Rng;        // bringing trait into scope
use chrono::Local;

fn main() {
    println!("-- Randomized greeting --");
    let li = ["Hello", "Hi", "Welcome", "Yo"];
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..li.len());
    println!("{}", li[n]);
    
    let now = Local::now();
    println!("Current time is {}", now);
}
