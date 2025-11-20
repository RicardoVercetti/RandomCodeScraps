#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u32,
    pub is_borrowed: bool,
}

impl Book {
    pub fn new(title: String, author: String, year: u32) -> Self {
        Book {
            title,
            author,
            year,
            is_borrowed: false,
        }
    }
    
    pub fn borrow(&mut self) -> bool {
        if self.is_borrowed {
            false
        } else {
            self.is_borrowed = true;
            true
        }
    }
    
    pub fn return_book(&mut self) -> bool {
        if self.is_borrowed {
            self.is_borrowed = false;
            true
        } else {
            false
        }
    }
    
    pub fn edit_book(&mut self, title: String, author: String, year: u32) {
        // if an empty string is passed, it means keep the old data
        if title.len() > 0 {
            self.title = title;
        }
        
        if author.len() > 0 {
            self.author = author;
        }
        
        if year == 0 {    // zero can't be a valid year
            self.year = year;
        }
    }
}
