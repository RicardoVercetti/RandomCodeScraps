use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // basic_example().await;
    tokio::join!(task1(), task2());
}

async fn task1() {
    for i in 1..=8 {
        println!("Task 1: {}", i);
        sleep(Duration::from_millis(500)).await;
    }
}

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
