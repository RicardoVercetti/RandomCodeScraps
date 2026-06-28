use crate::stack::Stack;

pub fn divide_by_two(num: i32) -> String {
    let mut stack = Stack::new();
    let mut ntw = num.clone();
    while ntw > 0 {
        // keep dividing and pushing into the stack
        let rem = ntw % 2;
        ntw /= 2;
        stack.push(rem);
    }

    let final_collection = stack.into_iter().map(|c| c.to_string()).collect::<String>();
    final_collection
}

pub fn divide_by_sixteen(num: i32) -> String {
    let hex_values = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
    let mut stack = Stack::new();
    let mut ntw = num.clone();
    while ntw > 0 {
        let rem = ntw % 16;
        ntw /= 16;
        stack.push(rem as usize);
    }

    let mut final_out = "".to_string();
    while !stack.is_empty() {
        let hex_val = hex_values.get(stack.pop().unwrap()).unwrap();
        final_out += &hex_val.to_string();
    }
    final_out
}
