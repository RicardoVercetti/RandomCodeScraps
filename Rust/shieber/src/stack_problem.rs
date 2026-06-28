use crate::stack::Stack;

pub fn is_valid_bracket(s: &str) -> bool {
    let mut stack = Stack::new();
    let chars = s.chars();
    for char in chars {
        if char == '(' {
            stack.push(char);
        } else if char == ')' {
            if stack.is_empty() {
                return false;
            }

            stack.pop();
        }
    }

    stack.is_empty()
}

pub fn get_corresponding_opening(s: char) -> char {
    if s == '}' {
        return '{';
    } else if s == ')' {
        return '(';
    } else if s == ']' {
        return '[';
    }

    panic!("you entered an invalid char: {s}");
}

pub fn is_valid_multi_brackets(s: &str) -> bool {
    let mut stack = Stack::new();
    for char in s.chars() {
        if char == '{' || char == '(' || char == '[' {
            stack.push(char);
        } else if char == '}'  || char == ')' || char == ']' {
            let item_at_top = stack.peek();
            if item_at_top.is_none() { return false };
            if item_at_top.unwrap() == &get_corresponding_opening(char) {
                stack.pop();
            } else {
                return false;
            }
        }
    }

    stack.is_empty()
}
