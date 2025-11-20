
//mod book;

use crate::book::Book;

pub struct Library {
    books: Vec<Book>,   
}

impl Library {
    pub fn new() -> Self {
        Library { books: Vec::new() }
    }
    
    pub fn add_book(&mut self, b: Book) {
        self.books.push(b);
    }
    
    pub fn show_all_books(&self) {
        for (i, book) in self.books.iter().enumerate() {
            println!("{}. {}({}) - {} [{}]", i+1, book.title, book.year, book.author,
                        if book.is_borrowed { "Borrowed" }  else { "Available" });
        }
    }
    
    pub fn no_of_books(&self) -> usize {
        self.books.len()
    }
    
    pub fn get_iter_ref(&self) -> &Vec<Book> {
        &self.books
    }
    
    pub fn get_iter_mut_ref(&mut self) -> &mut Vec<Book> {
        &mut self.books
    }
    
    pub fn find_by_author(&self, author: &str) {
        let filtered: Vec<&Book> = self.books.iter().filter(|book| book.author.contains(author)).collect();
        for (i, book) in filtered.iter().enumerate() {
            println!("{}. {}({}) - {} [{}]", i+1, book.title, book.year, book.author,
                        if book.is_borrowed { "Borrowed" }  else { "Available" });
        }
        
        if filtered.is_empty() {
            println!("No books found for the author: {}", author);
        }
    }
    
}

