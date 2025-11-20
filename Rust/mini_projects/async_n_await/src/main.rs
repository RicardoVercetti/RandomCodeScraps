// Tasks
// 1. Write an async program with two async functions:
// 2. One prints numbers 1–5 with a delay.
// 3. Another prints letters a–e with a delay.
// 4. Run them concurrently with tokio::join!.
// 5. Bonus: Make a function that simulates fetching data from an API (just sleep + print "Data fetched").

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // basic_example().await;
    // tokio::join!(task1(), task2());
    tokio::join!(print_numbers(), print_letters(), pretend_to_fetch_data_from_api());
}

async fn print_numbers() {
    for i in 1..=5 {
        println!("Num : {}", i);
        sleep(Duration::from_millis(500)).await;
    }
}

async fn print_letters() {
    let alphs = vec!["a", "b", "c", "d", "e"];
    for alph in alphs {
        println!("Alph : {}", alph);
        sleep(Duration::from_millis(500)).await;
    }
}

async fn exchange_data() -> String {
    println!("Sending data...");
    println!("Data sent");
    println!("Waiting for response...");
    sleep(Duration::from_millis(600)).await;
    println!("Received data...");
    println!("Returning response...");
    sleep(Duration::from_millis(200)).await;
    "Some response".to_string()
}

async fn pretend_to_fetch_data_from_api() {
    sleep(Duration::from_millis(300)).await;
    let resp = exchange_data().await;
    println!("Response received : {}", resp);
}

#[allow(dead_code)]
async fn task1() {
    for i in 1..=8 {
        println!("Task 1: {}", i);
        sleep(Duration::from_millis(500)).await;
    }
}

#[allow(dead_code)]
async fn task2() {
    for i in 1..=5 {
        println!("Task 2: {}", i);
        sleep(Duration::from_millis(700)).await;
    }
}

#[allow(dead_code)]
async fn basic_example() {
    say_hello().await;
}

#[allow(dead_code)]
async fn say_hello() {
    println!("Hello...");
    sleep(Duration::from_secs(1)).await;
    println!("...world!");
}
