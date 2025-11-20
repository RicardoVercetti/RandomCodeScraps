// 1. add math.rs in src/
// 2. implement add, subtract, multiply, divide
// 3. make them pub
// 4. in main.rs import mod math and use these functions
// 5. Extend it, create another file greet.rs with a function hello(name: &str)
// 6. Call both from main.rs

mod math;
use calc_project::utils;

fn main() {
    println!("Het there...!!!");
    utils::greet("Alice");
    
    let a: f32 = 3.4;
    let b: f32 = 5.6;
    
    println!("Result add: {}", math::add(a,b));
}
