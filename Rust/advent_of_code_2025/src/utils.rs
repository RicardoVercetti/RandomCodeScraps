use std::{fs::File, io::Read};

#[allow(dead_code)]
pub fn get_file_contents(s: &str) -> String {
    let mut file = File::open(s).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content
}