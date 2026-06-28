use std::fmt::Debug;

// stack
// methods: new, push, pop, peek, is_empty, size, iter, iter_mut, into_iter

#[derive(Debug)]
pub struct Stack<T> {
    size: usize,
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { size: 0, items: Vec::new() }
    }

    pub fn push(&mut self, new_item: T) {
        self.size += 1;
        self.items.push(new_item);
    }

    pub fn pop(&mut self) -> Option<T> {
        let item = self.items.pop();

        if item.is_some() {
            self.size-=1;
        }
        item
    }

    pub fn peek(&self) -> Option<&T> {
        if self.size < 1 {
            return None;
        }
        self.items.get(self.size - 1)
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    // todo: peek_mut()

    // Implementing iterator for stack
    // into_iter: stack modifier and becomes an iterator
    // iter: stack unmodified and get a un-mutable iterator
    // iter_mut: stack unmodified and get a mutable  iterator

    pub fn into_iter(self)  -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.items.iter() {         // note: isn't this kinda in efficient?
            println!("pushing to vec in iter...");
            iterator.stack.push(item);
        }

        iterator
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.items.iter_mut() {
            println!("pushing vec in iter_mut...");
            iterator.stack.push(item);
        }

        iterator
    }


}


// Iterator implementations
pub struct IntoIter<T>(Stack<T>);
impl <T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            println!("calling next at: {}", self.0.size);
            self.0.size -= 1;
            self.0.items.pop()
        } else {
            None
        }
    }
}

pub struct Iter<'a, T: 'a> { stack: Vec<&'a T>, }       // life time of Iter is atleast as long as 'a, lifetime of T is atleast as long as 'a
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

pub struct IterMut<'a, T: 'a> { stack: Vec<&'a T> }
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            self.stack.pop()
        }
}

// impl <T: Clone> Iterator for Stack<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.size() != 0 {
//             println!("some other iterator at: {}", self.size);
//             self.size -= 1;
//             self.pop()
//         } else {
//             None
//         }
//     }
// }