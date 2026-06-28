// scalar types     : integer, float, boolean, character
// composite types  : tuple, array 

use shieber::stack::Stack;

#[allow(dead_code)]
fn stack_and_stroll() {
    let mut my_stack = Stack::new();

    // operations: 
    // 1. is empty
    println!("1. is_empty: {}, expected: true", my_stack.is_empty());

    // 2. push(1)
    my_stack.push(1);

    // 3. push(2)
    my_stack.push(2);

    // 4. peek()    -> 2
    println!("4. peek: {:?}, expected: 2", my_stack.peek());

    // 5. push(3)
    my_stack.push(3);

    // 6. size()    -> 3
    println!("6. size: {}, expected: 3", my_stack.size());

    // 7. is_empty()
    println!("7. is_empty: {}, expected: false", my_stack.is_empty());

    // 8. push(4)
    my_stack.push(4);

    // 9. push(5)
    my_stack.push(5);

    // 10. size()   -> 5
    println!("10: size: {}, expected: 5", my_stack.size());

    // 11. pop()    -> 5
    println!("11. pop: {:?}, expected: 5", my_stack.pop());

    // 12. push(6)
    my_stack.push(6);

    // 13. peek()   -> 6
    println!("13. peek: {:?}, expected: 6", my_stack.peek());
}

fn main() {
    println!("helo, russians?");

    let mut my_stack = Stack::new();

    my_stack.push(1);
    my_stack.push(2);
    my_stack.push(3);

    // let itera = my_stack.into_iter();
    // for it in itera {
    //     println!("itering: {}", it);
    // }

    // for item in my_stack {
    //     println!("item is: {}", item);
    // }

    // println!("here comes the money: {:?}", my_stack);

    // let into_iter = my_stack.into_iter();
    // for item in into_iter {
    //     println!("item: {}", item);
    // }

    // let iter = my_stack.iter();
    // for item in iter {
    //     println!("item: {}", item);
    // }

    let iter_mut = my_stack.iter_mut();
    for item in iter_mut {
        println!("item: {}", item);
    }
}
