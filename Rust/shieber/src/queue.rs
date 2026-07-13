// queue implementation

// is_empty
// enqueue -> added at the end
// dequeue -> removed from the start
// size


pub struct Queue<T> {
    data: Vec<T>,       // todo: should've used a fixed length type
    size: usize
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self { data: Vec::new(),
            size: 0 as usize 
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn enqueue(&mut self, new_item: T) {
        self.data.push(new_item);
        self.size += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let pos = 0;
        let return_data = self.data.remove(pos);        // todo: builtin function's doing the heavy lifting here
        self.size -= 1;
        Some(return_data)
    }

    pub fn size(&self) -> usize {
        self.size
    }

}