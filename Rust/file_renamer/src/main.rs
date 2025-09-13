// file reneamer

// 1. take input for directory location
// 2. take input for pattern
// 3. get all the files from the directory
// 4. rename one by one with pattern and numbers incrementally(eg: vacation-001.jpg, vacation-002.jpg, etc.,)


// Some(LLMs)


use std::fs;

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
    
    let files = get_files_in_folder(".").unwrap();
    
    
    for file in files {
        println!("{:?}", file.path());
    }
    //println!("Finished writing file");
}
