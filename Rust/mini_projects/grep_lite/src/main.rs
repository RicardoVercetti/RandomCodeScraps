// Grep lite - no chatgpt allowed, just google in the old school way
// take an input for word2search & file from CLI
// search the text line by line
// when a match is found, print line with the line number

//use std::io;
use std::fs;
use colored::Colorize;
use std::env;

fn extract_params(v: Vec<String>) -> (String, String) {
    let one = v.get(1).expect("arg 1 is absent");
    let two = v.get(2).expect("arg 2 is absent");
    
    (one.to_string(), two.to_string())    
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let (query, file_path) = extract_params(args);
    
    println!("Search for {query}");
    println!("In file {file_path}");
    
    
    // check if file exists,
    let contents = fs::read_to_string(file_path)
                        .expect("Cant open file, does the file exist?");
        
    
    // if does, start looping through line by line
    for (i, line) in contents.split("\n").enumerate() {
        //println!("{} - '{}'", i, hex::encode(item));
        if line.contains(&query) {
            println!("//{} {}", i, line.red());
        }
    }
    
    
    println!("Goodbye...");
}
