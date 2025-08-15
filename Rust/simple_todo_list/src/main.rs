// A simple To-Do list app
// 1. lets you add tasks
// 2. lets you list all tasks
// 3. saves tasks to a file
// 4. loads tasks from the file on startup


// Things to learn
// structs
// Vec
// File I/O with std::fs
// Serialization to simple txt format line by line

//use std::io;
use std::fs::OpenOptions;
use std::fs::read_to_string;
use std::io::Write;
use std::io;

//#[derive(Debug)]
struct Task {
    description: String,
}

impl Task {
    fn new(desc: String) -> Self {
        Task {
            description: desc
        }
    }
    
    fn display_data(&self) {
        println!("Task desc : {}", self.description);
    }
}

fn load_tasks() -> Vec<Task> {
    let mut tasks = Vec::new();
    if let Ok(contents) = read_to_string("tasks.txt") {
        for line in contents.lines() {
            tasks.push(Task::new(line.to_string()));
        }
    }
    tasks
}

fn save_tasks(tasks: &Vec<Task>) {
    let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("tasks.txt")
            .expect("Could not open file");
            
            for task in tasks {
                writeln!(file, "{}",task.description)
                    .expect("Could not write to file");
            }
}

fn main() {
    println!("Heyy, this runs :) ");
    
    loop {
        
        println!("\n--- To-Do List ---");
        
        ////println!("-- Available tasks --");
        //for task in load_tasks() {
        //    task.display_data();
        //}
        
        println!("H: 1 for adding tasks, 2 for viewing tasks, q for quit.");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        let choice = choice.trim();
        
        match choice {
            "1" => {    // add a task
                let mut input = String::new();
                println!("Enter the task description : ");
                io::stdin().read_line(&mut input).unwrap();
                
                let mut tasks = load_tasks();
                tasks.push(Task::new(input.trim().to_string()));
                save_tasks(&tasks);
                println!("Task added!");
                
            },
            "2" => { // List tasks
                println!("--------------------");
                for (i, task) in load_tasks().iter().enumerate() {
                    println!("{}: {}", i+1, task.description);
                }
                println!("--------------------");
                //println!("-- thats all there is --");
            },
            "q" => {
                println!("Goodbye...!");
                break;
            },
            
            _ => {
                println!("Invalid value, please enter again....");
                continue;
            }
        }
        
    }
    
}
