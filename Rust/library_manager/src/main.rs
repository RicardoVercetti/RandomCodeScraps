// Libray manager

// 1. Add a book
// 2. List all books
// 3. Borrow a book(mark it as borrowed) 
// 4. Return a book

use std::io;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    is_borrowed: bool,
}

impl Book {
    fn new(title: String, author: String) -> Self {
        Book {
            title,
            author,
            is_borrowed: false,
        }
    }
    
    fn borrow(&mut self) -> bool {
        if self.is_borrowed {
            false
        } else {
            self.is_borrowed = true;
            true
        }
    }
    
    fn return_book(&mut self) -> bool {
        if self.is_borrowed {
            self.is_borrowed = false;
            true
        } else {
            false
        }
    }
}

fn main() {
    
    let mut books: Vec<Book> = Vec::new();
    
    'mainloop: loop {
        println!("--- Library Manager ---");
        println!("1: Add book, 2: List Book, 3: Borrow Book, 4: Return book, q: Quit");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        
        match choice {
            "1" => {
                // add a book
                let mut title = String::new();
                println!("Enter the Title: ");
                io::stdin().read_line(&mut title).unwrap();
                let title = title.trim();
                
                let mut author = String::new();
                println!("Enter the Author Name: ");
                io::stdin().read_line(&mut author).unwrap();
                let author = author.trim();
                
                books.push(Book::new(title.to_string(), author.to_string()));
                println!("Book added successfully");
            },
            "2" => {
                // list books
                for (i, book) in books.iter().enumerate() {
                    println!("{}. {} - {} [{}]", i+1, book.title, book.author, 
                        if book.is_borrowed { "Borrowed" }  else { "Available" });
                }
                
                if books.len() == 0 {
                    println!("No books to show");
                }
            },
            "3" => {
                // Borrow book
                println!("Enter the ID to borrow:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id = id.trim().parse::<usize>().unwrap();
                
                
                for (i, book) in books.iter_mut().enumerate() {
                    if i+1 == id {
                        //println!("The compared val: {}", i+1);
                        if book.borrow() {
                            println!("You've borrowed: {}", book.title);
                            continue 'mainloop;
                        }
                        println!("This book is already borrowed, cant be borrowed again!");
                        continue 'mainloop;
                    }
                }
                
                println!("Can't find any book with ID: {}", id);
            },
            "4" => {
                // Retrun book
                println!("Enter the ID to return: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id = id.trim().parse::<usize>().unwrap();
                
                if id > books.len() {
                    println!("Invalid book ID: {}", id);
                    continue 'mainloop;
                }
                
                for (i, book) in books.iter_mut().enumerate() {
                    if i+1 == id {
                        if book.return_book() {
                            println!("You've successfully returned the book: {}", book.title);
                            continue 'mainloop;
                        }
                        println!("This book is already returned, therefore can't be returned");
                        continue 'mainloop;
                    }
                }
                println!("Wait! how'd you enter this part of the code?!!!");
            },
            "q" => {
                println!("Goodbye...");
                break;
            },
            _ => println!("Invalid choice!")
        }
    }
    
}

