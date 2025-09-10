// Grep lite - no chatgpt allowed, just google in the old school way
// take an input for
// 1 - word to search
// 2 - file location in which to search
// search the text line by line
// when a match is found, print line with the line number

use std::io;
use std::fs;
use colored::Colorize;

fn main() {
    println!("Word to search:");
    let mut word = String::new();
    io::stdin().read_line(&mut word).unwrap();
    let word = word.trim();
    
    println!("Enter the file location:");
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).unwrap();
    let file_name = file_name.trim();
    
    println!("Word is: {}", word);
    println!("File name: {}", file_name);
    
    // check if file exists,
    let contents = fs::read_to_string(file_name)
                        .expect("Cant open file, does the file exist?");
        
    
    // if does, start looping through line by line
    for (i, line) in contents.split("\n").enumerate() {
        //println!("{} - '{}'", i, hex::encode(item));
        if line.contains(word) {
            println!("//{} {}", i, line.red());
        }
    }
    
    
    println!("Goodbye...");
}
