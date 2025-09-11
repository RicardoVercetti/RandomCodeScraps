use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
}

const STORAGE_FILE: &str = "people.json";

fn load_data() -> Vec<Person> {
    if let Ok(data) = fs::read_to_string(STORAGE_FILE) {
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

fn save_data(data: &Vec<Person>) {
    let json_str = serde_json::to_string_pretty(data).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(STORAGE_FILE)
        .unwrap();
    file.write_all(json_str.as_bytes()).unwrap();
}

fn main() {
    println!("-- Welcome to the Serde CLI --");

    let mut people = load_data();

    loop {
        let mut inp = String::new();
        println!("\nEnter 1: add object, 2: List all objects, 3: exit");
        println!("Enter input: ");
        io::stdin().read_line(&mut inp).unwrap();
        let inp = inp.trim();

        match inp {
            "1" => {
                // Add object
                let mut name = String::new();
                println!("Enter name: ");
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                let mut age_str = String::new();
                println!("Enter age: ");
                io::stdin().read_line(&mut age_str).unwrap();
                let age: u8 = age_str.trim().parse().unwrap_or(0);

                let person = Person { name, age };
                people.push(person);
                save_data(&people);

                println!("Person added!");
            }
            "2" => {
                // List all objects
                if people.is_empty() {
                    println!("No records found.");
                } else {
                    println!("-- People List --");
                    for (i, person) in people.iter().enumerate() {
                        println!("{}. {:?} ", i + 1, person);
                    }
                }
            }
            "3" => {
                println!("Goodbye...");
                break;
            }
            _ => println!("Invalid option. Try again."),
        }
    }
}
