use std::collections::HashMap;

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
    
    let counts = word_count(text);
    
    println!("Word frequencies:");
    for (word, count) in counts.iter() {
        println!("{}: {}", word, count);
    }
}
