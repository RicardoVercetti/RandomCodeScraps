// Notes manager
// 1. Lets you add notes with a title + content
// 2. Lets you list all notes
// 3. Lets you search notes by keyword
// 4. Saves to files and loads on startup

// Things to learn
// Enums to represents menu options
// Structs with multiple fields (title + content)
// Searching in a Vec using iter() and .filter()
// More practice with ownership & borrowing

use std::fs::{OpenOptions, read_to_string};
use std::io::{self,Write};

#[derive(Debug)]
struct Note {
    title: String,
    content: String,
}

impl Note {
    fn new(title: String, content: String) -> Self {
        Note { title, content }
    }
}

fn save_notes(notes: &Vec<Note>) {
    let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open("notes.txt")
                    .expect("Could not open file");
                    
    for note in notes {
        writeln!(file, "{}", note.title).unwrap();
        writeln!(file, "{}", note.content).unwrap();
    }
    println!("File saved successfully");
}

fn load_notes() -> Vec<Note> {
  let mut notes = Vec::new();
  if let Ok(contents) = read_to_string("notes.txt") {
      let mut lines = contents.lines();
      while let (Some(title), Some(content)) = (lines.next(), lines.next()) {
          notes.push(Note::new(title.to_string(), content.to_string()));
      }
  }
  notes
}

fn main() {
    let mut notes = load_notes();
    
    loop {
        println!("-- Welcome to Notes manager --\n");
        println!("1: Add note, 2: List notes, 3: Search notes, q: Quit");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        let choice = choice.trim();
        
        match choice {    // Add note
            "1" => {
                let mut title = String::new();
                println!("Enter note title:");
                io::stdin().read_line(&mut title).unwrap();
                
                let mut content = String::new();
                println!("Enter note content:");
                io::stdin().read_line(&mut content).unwrap();
                
                let note = Note::new(title.trim().to_string(), content.trim().to_string());
                notes.push(note);
                save_notes(&notes);
                println!("Note added");
            },
            "2" => {    // List notes
                for (i, note) in notes.iter().enumerate() {
                    println!("{}. {} - {}", i+1, note.title, note.content);
                }
            },
            "3" => {
                 let mut key = String::new();
                 println!("Enter a key word to search:");
                 io::stdin().read_line(&mut key).unwrap();
                 let key = key.trim().to_lowercase();
                 
                 println!("\nSearch results:");
                 for note in notes.iter().filter(|n| 
                         n.title.to_lowercase().contains(&key) ||
                         n.content.to_lowercase().contains(&key)
                    ) {
                        println!("{} - {}", note.title, note.content);
                    }
                 
            },
            "q" => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Invalid choice!"),
            
        }
        
    }
    
}
