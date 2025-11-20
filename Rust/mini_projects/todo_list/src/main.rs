use std::io;
use std::collections::VecDeque;

fn main() {
    let mut tasks: VecDeque<String> = VecDeque::new();

    loop {
        println!("\nTo-Do List:");
        for (i, task) in tasks.iter().enumerate() {
            println!("{}. {}", i + 1, task);
        }
        println!("\nOptions:");
        println!("1. Add a task");
        println!("2. Remove a task");
        println!("3. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice = input.trim();

        match choice {
            "1" => {
                println!("Enter task:");
                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("Failed to read input");
                tasks.push_back(task.trim().to_string());
            }
            "2" => {
                println!("Enter task number to remove:");
                let mut num = String::new();
                io::stdin().read_line(&mut num).expect("Failed to read input");

                if let Ok(index) = num.trim().parse::<usize>() {
                    if index > 0 && index <= tasks.len() {
                        tasks.remove(index - 1);
                        println!("Task removed!");
                    } else {
                        println!("Invalid task number.");
                    }
                } else {
                    println!("Invalid input.");
                }
            }
            "3" => break,
            _ => println!("Invalid option, try again."),
        }
    }
}
