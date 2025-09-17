// file reneamer

// 1. take input for directory location
// 2. take input for pattern
// 3. get all the files from the directory
// 4. rename one by one with pattern and numbers incrementally(eg: vacation-001.jpg, vacation-002.jpg, etc.,)


// Some(LLMs)

use std::env;
use std::fs;
use std::path::{ PathBuf, Path };

fn get_files_in_folder(path: &str) -> Result<Vec<fs::DirEntry>, std::io::Error> {
    let entries = fs::read_dir(path)?;
    let mut files = Vec::new();
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        
        if path.is_file() {
            files.push(entry);
        }
    }
    
    Ok(files)
}

fn main() {
    
    // get folder location input in cli
    // check if it exists & actually a folder
    // get pattern, not special charecters
    
    let args_vec: Vec<String> = env::args().collect();
    
    //println!("All args : {:?}", args_vec);
    let folder_loc = args_vec.get(1).expect("Folder location must be passed in!");
    let file_name_pattern = args_vec.get(2).expect("Pattern must be passed!");
    
    //println!("Folder loc: {}", folder_loc);
    //println!("File name: {}", file_name_pattern);
    
    let folder = Path::new(folder_loc);
    
    if !folder.exists() || !folder.is_dir() {
        println!("The folder {} doesn't exist", folder_loc);
        return;
    }
    
    if file_name_pattern.chars().any(|a| !a.is_alphanumeric()) {
        println!("file name should be alphanumeric");
        return;
    }
    
    let files = get_files_in_folder(folder_loc).unwrap();
    
    
    // renaming of the files
    for (i, file) in files.iter().enumerate() {
        if let Some(ext) = file.path().extension() {
            if let Some(parent) = file.path().parent() {
                let new_name = format!(
                    "{}_{}.{}",
                    file_name_pattern,
                    i+1,
                    ext.to_string_lossy()
                );
                
                let new_path: PathBuf = parent.join(new_name);
                
                match fs::rename(&file.path(), &new_path) {
                    Ok(_) => println!("Renamed successfully for: {:?}", &file.path()),
                    Err(_) => println!("Failed to rename file: {:?}", &file.path()),
                }
            }
        }
    }
    println!("Done!");
}
