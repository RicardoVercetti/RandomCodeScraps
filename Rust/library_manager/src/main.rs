// Libray manager

// 1. Add a book
// 2. List all books
// 3. Borrow a book(mark it as borrowed) 
// 4. Return a book

// Tasks
// 5. Search books by title or author
// 6. Add a new field year: u32 to the Book
// 7. Add a Edit Book option and also include year in there

mod book;
mod library;

use book::Book;
use library::Library;


use std::io;


fn main() {
    
    let mut library = Library::new();
    
    'mainloop: loop {
        println!("--- Library Manager ---");
        println!("1: Add book, 2: Edit book, 3: List Book, \n4: Borrow Book, 5: Return book, 6: Search Book, q: Quit");
        
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
                
                let mut year = String::new();
                println!("Enter the Year: ");
                io::stdin().read_line(&mut year).unwrap();
                let year: u32 = match year.trim().parse() {
                    Ok(yr) => {
                        if yr == 0 {
                            println!("Year must be a non-zero value!");
                            continue 'mainloop;
                        }
                        yr
                    },
                    Err(_) => {
                        println!("Year have to be completely numerical!");
                        continue 'mainloop;
                    },
                };
                
                library.add_book(Book::new(title.to_string(), author.to_string(), year));
                println!("Book added successfully");
            },
            "2" => {
            todo!("edit is not supported anymore!");
              // Edit book
            //  let mut id = String::new();
            //  println!("Enter the ID to edit:");
            //  io::stdin().read_line(&mut id).unwrap();
              
            //  let id: usize = match id.trim().parse() {
            //      Ok(val) => val,
            //      Err(_) => {
            //          println!("Enter a valid ID");
            //          continue 'mainloop;
            //      }
            //  };
              
            //  if id > library.no_of_books() {
            //      println!("Such ID doesn't exist : {}", id);
            //      continue 'mainloop;
            //  }
              
              // take title, author, year
            //  let mut title = String::new();
            //  let mut author = String::new();
            //  let mut year = String::new();
              
            //  println!("Enter the new title: ");
            //  io::stdin().read_line(&mut title).unwrap();
            //  let title = title.trim();
              
            //  println!("Enter the new author name: ");
            //  io::stdin().read_line(&mut author).unwrap();
            //  let author = author.trim();
              
            //  println!("Enter the year: ");
            //  io::stdin().read_line(&mut year).unwrap();
            //  let year: u32 = match year.trim().parse() {
            //      Ok(val) => val,
            //      Err(e) => {
            //          println!("Error: {}", e);
            //          println!("Can't update with invalid year as input!");
            //          continue 'mainloop;
            //      }
            //  };
              
            //  for (i, book) in books.iter_mut().enumerate() {
            //      if id == i+1 {
            //          book.edit_book(title.to_string(), author.to_string(), year);
            //          println!("Updated successfully...");
              //        continue 'mainloop;
            //      }
              //}
              //println!("No book found! Updation failed!");
                             
            },
            "3" => {
                // list books
                library.show_all_books();
                
                if library.no_of_books() == 0 {
                    println!("No books to show");
                }
            },
            "4" => {
                // Borrow book
                println!("Enter the ID to borrow:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id = id.trim().parse::<usize>().unwrap();
                
                
                for (i, book) in library.get_iter_mut_ref().iter_mut().enumerate() {
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
            "5" => {
                // Retrun book
                println!("Enter the ID to return: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id = id.trim().parse::<usize>().unwrap();
                
                if id > library.no_of_books() {
                    println!("Invalid book ID: {}", id);
                    continue 'mainloop;
                }
                
                for (i, book) in library.get_iter_mut_ref().iter_mut().enumerate() {
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
            "6" => { // case insensitive search
                let mut text = String::new();
                println!("Enter the text to search: ");
                io::stdin().read_line(&mut text).unwrap();
                
                let text = text.trim().to_lowercase();
                println!("Search results:");
                let filtered_books: Vec<&Book> = library.get_iter_mut_ref().iter().filter(|&book| 
                book.title.to_lowercase().contains(&text) ||
                book.author.to_lowercase().contains(&text)).collect();
                
                for book in &filtered_books {
                    println!("Book: {}, Author: {}, Year: {}", book.title, book.author, book.year);
                }
                
                if filtered_books.is_empty() {
                    println!("No results found!");
                }
            },
            "q" => {
                println!("Goodbye...");
                break;
            },
            _ => println!("Invalid choice!")
        }
    }
    
}

