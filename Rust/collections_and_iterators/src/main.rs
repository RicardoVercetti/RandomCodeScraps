#[allow(dead_code)]
fn vectors() {
    let mut nums = Vec::new();
    nums.push(10);
    nums.push(20);
    
    println!("First: {:?}", nums[0]);
    println!("All: {:?}", nums);
}

use std::collections::HashMap;

#[allow(dead_code)]
fn hash_map() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 10);
    scores.insert("Bob", 20);
    
    println!("{}", scores.get("Alice").copied().map_or("None".to_string(), |score| score.to_string()));
}

fn main() {
    println!("Hey there!");
    
    let nums= vec!(1, 2, 3, 4, 5, 6);
    
    let squares: Vec<i32> = nums.iter().map(|x| x*x).collect();
    println!("{:?}", squares);
    println!("{:?}", nums);
}
