// file reneamer

// 1. take input for directory location
// 2. take input for pattern
// 3. get all the files from the directory
// 4. rename one by one with pattern and numbers incrementally(eg: vacation-001.jpg, vacation-002.jpg, etc.,)


// No LLMS, only docs and google!

use std::io::{ stdin, Write };
use std::fs;

fn main() {
    println!("-- Welcome to file renamer --");
    
    // lets go with user input for now
    println!("Enter dir location: ");
    let mut dir = String::new();
    stdin().read_line(&mut dir).unwrap();
    let dir = dir.trim();
    
    println!("Enter pattern: ");
    let mut pat = String::new();
    stdin().read_line(&mut pat).unwrap();
    let pat = pat.trim();
    
    //println!("Dir: '{}'", dir);
    //println!("Pat: '{}'", pat);
    
    // files.
    
    //let mut file = File::create("foo.txt").unwrap();
    //file.write_all(b"Hello eta-verse!");
    
    let paths = fs::read_dir("./").unwrap();
    
    for path in paths {
        //println!("Name: {}", path.unwrap().path().display());
        println!("Metadata : {:?}", path.unwrap().metadata().unwrap())
    }
    
    
    //println!("Finished writing file");
}
