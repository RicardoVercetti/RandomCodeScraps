// linked lists are somewhat like arrays that store data in a distributed manner in the memory. 
// they are not stord in the continuous manner as in the treditional arrays
// each item has a pointer to the next item in the linked list


#[derive(Debug)]
struct LinkedList {
    first_item: NextNode
}

impl LinkedList {
    fn new() -> Self {
        Self { first_item: NextNode::None }
    }

    fn show(&self) {
        println!("{:?}", &self.first_item);
    }

    fn push(&mut self, nextNode: NextNode) {
        match &self.first_item {
            NextNode::None => self.first_item = nextNode,
            NextNode::Node(_) => {
                // loop through all items and find the last item, then push there
                println!("there are item(s) already in the node, gotta find the last element");
                todo!("unimplemented");
            }
        }
        
    }
}

#[derive(Debug)]
struct Node {
    this_number: i32,
    next_number: NextNode
}

#[derive(Debug)]
enum NextNode {
    None,
    Node(Box<Node>),
}

fn main() {
    println!("here is my custom implementation of linked lists...");
    let mut lili = LinkedList::new();
    println!("{:?}", lili);

    lili.push(NextNode::Node({ this_number: 1, next_number: NextNode::None }));

    println!("{:?}", lili);

    // pushing first item
}
