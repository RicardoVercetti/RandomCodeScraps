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

fn main() {
    println!("-- Welcome to Notes manager --");
    let note = Note::new("Note 1".to_string(), "Gotta do something".to_string());
    
    println!("N1|C1 => {}|{}", note.title, note.content);
}
