// scalar types     : integer, float, boolean, character
// composite types  : tuple, array 

use shieber::stack::Stack;
use shieber::stack_problem::{
    is_valid_bracket,
    is_valid_multi_brackets
};
use shieber::decimal_conversions::{
    divide_by_sixteen,
    divide_by_two
};

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

#[allow(dead_code)]
fn stack_exercises() {
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

#[allow(dead_code)]
fn valid_parenthesis() {
    let brackets = String::from("()(())");
    let brackets2 = String::from("()((()");

    let result1 = is_valid_bracket(&brackets);
    let result2 = is_valid_bracket(&brackets2);

    let brackets3 = String::from("{ { ( [ ] [ ] ) } ( ) }");
    let brackets4 = String::from("( ( ( ) ] ) )");

    let result3 = is_valid_multi_brackets(&brackets3);
    let result4 = is_valid_multi_brackets(&brackets4);

    println!("res1: {result1}");
    println!("res2: {result2}");
    println!("res3: {result3}");
    println!("res4: {result4}");
}

#[allow(dead_code)]
fn decimal_conversions() {
    let num = 233;
    let num1 = 10;
    let num2 = 43;
    let result = divide_by_two(num);
    let result2 = divide_by_sixteen(num);
    let result3 = divide_by_two(num1);
    let result4 = divide_by_sixteen(num2);
    
    println!("final collection: {}", result);
    println!("final collection2: '{}'", result2);

    println!("num3: {}", result3);
    println!("num4: {}", result4);
}

fn main() {
    println!("helo, russians?");

    decimal_conversions();
}
