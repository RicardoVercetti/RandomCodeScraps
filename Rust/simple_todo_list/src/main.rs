// A simple To-Do list app
// 1. lets you add tasks
// 2. lets you list all tasks
// 3. saves tasks to a file
// 4. loads tasks from the file on startup

// Task
// 5. Each task should have status
// 6. Menu option to mark a status as done


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
use std::fmt;

enum TaskStatus {
    Pending,
    Done,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "PENDING"),
            TaskStatus::Done => write!(f, "DONE"),
        }
    }
}

fn get_d_enum(en: &str) -> TaskStatus {
    match en {
        "PENDING" => TaskStatus::Pending,
        "DONE" => TaskStatus::Done,
        _ => TaskStatus::Pending,
    }
}


//#[derive(Debug)]
struct Task {
    description: String,
    status: TaskStatus,
}

impl Task {
    fn new(desc: String, status: TaskStatus) -> Self {
        Task {
            description: desc,
            status: status,
        }
    }
    
    #[allow(dead_code)]
    fn display_data(&self) {
        println!("Task|Status : {}|{}", self.description, self.status);
    }
}

fn load_tasks() -> Vec<Task> {
    let mut tasks = Vec::new();
    if let Ok(contents) = read_to_string("tasks.txt") {
        for line in contents.lines() {
            let (desc, status) = line.split_once(",")
                    .expect("Invalid input from the saved file");
            tasks.push(Task::new(desc.to_string(), get_d_enum(status.trim())));
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
                writeln!(file, "{}, {}",task.description, task.status)
                    .expect("Could not write to file");
            }
}


fn toggle_status(task_ref: &mut Task) {
    match task_ref.status {
        TaskStatus::Pending => {
            task_ref.status = TaskStatus::Done;
        },
        TaskStatus::Done => {
            task_ref.status = TaskStatus::Pending;
        },
    }
}

//fn set_task_status(task: String, status: TaskStatus, tasks: &mut Vec<Task>) {
    // iter through it and set the status
//    if let Some(task) = tasks.iter().find(|t| t.description == task) {
//        println!("Found: {}|{}", task.description, task.status);
//    }
        
//}

fn main() {
    
    println!("--- To-Do List ---\n");
        
    loop {
        println!("H: 1 for adding tasks, 2 for viewing tasks, 3 for change status, q for quit.");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        let choice = choice.trim();
        
        match choice {
            "1" => {    // add a task
                let mut input = String::new();
                println!("Enter the task description : ");
                io::stdin().read_line(&mut input).unwrap();
                
                
                let mut tasks = load_tasks();
                tasks.push(Task::new(input.trim().to_string(), TaskStatus::Pending));
                save_tasks(&tasks);
                println!("Task added!");
                
            },
            "2" => { // List tasks
                println!("--------------------");
                for (i, task) in load_tasks().iter().enumerate() {
                    println!("{}: {} | {}", i+1, task.description, task.status);
                }
                println!("--------------------");
                //println!("-- thats all there is --");
            },
            "3" => {    // change status
                println!("Enter id to change status: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: i32 = match id.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("please enter a valid number!");
                        continue;
                    }
                };
                
                let mut tasks = load_tasks();
                
                // toggle status
                for (i, task) in tasks.iter_mut().enumerate() {
                    if (i+1) == id.try_into().unwrap() {
                        toggle_status(task);
                        println!("Status change successful");
                    }
                }
                
                for task in &tasks {
                    println!("{}|{}", task.description, task.status);
                }
                
                save_tasks(&tasks);
                
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
