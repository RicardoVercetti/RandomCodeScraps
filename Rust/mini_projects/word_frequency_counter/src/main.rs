use std::collections::HashMap;

// Task 1 - ignore common stop words
// Task 2 - print in descending order

fn word_count(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    
    for word in text.split_whitespace().filter(|word| word != &"and" && 
                        word != &"is" && word != &"the") {
        let word = word.to_lowercase();
        *counts.entry(word).or_insert(0) += 1;
    }
    
    counts
}

fn main() {
    println!("Hello, world!");
    
    let text = "Rust is fast and Rust is safe and memory safe";
    
    let mut counts: Vec<(String, usize)> = word_count(text).into_iter().collect();
    counts.sort_by(|a, b| a.1.cmp(&b.1));
    counts.reverse();
    
    println!("Word frequencies:");
    for (word, count) in counts.iter() {
        println!("{}: {}", word, count);
    }
}
