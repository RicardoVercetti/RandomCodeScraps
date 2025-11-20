#[derive(Debug)]
struct LinkedList {
    arr: Vec<Node>
}

impl LinkedList {
    fn new() -> Self {
        Self { arr: Vec::new() }
    }

    fn show(&self) {
        println!("{:?}", &self.arr);
    }

    fn push(&mut self, nextNode: NextNode) {
        // if there is already some last node, make it point to this node and push this node, else just push this node
        
    }
}

#[derive(Debug)]
struct Node {
    this_number: i32,
    next_number: NextNode
}

#[derive(Debug)]
enum NextNode {
    NONE,
    Node
}

fn main() {
    println!("here is my custom implementation of linked lists...");
    let lili = LinkedList::new();
    println!("{:?}", lili);

    // pushing first item
}
