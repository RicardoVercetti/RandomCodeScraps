// file reneamer

// 1. take input for directory location
// 2. take input for pattern
// 3. get all the files from the directory
// 4. rename one by one with pattern and numbers incrementally(eg: vacation-001.jpg, vacation-002.jpg, etc.,)


// Some(LLMs)


use std::fs;
use std::path::PathBuf;

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
    
    let files = get_files_in_folder("./fldr/").unwrap();
    
    
    // renaming of the files
    for (i, file) in files.iter().enumerate() {
        if let Some(ext) = file.path().extension() {
            if let Some(parent) = file.path().parent() {
                let new_name = format!(
                    "renamed_{}.{}",
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
    println!("Finished renaming file...");
}
