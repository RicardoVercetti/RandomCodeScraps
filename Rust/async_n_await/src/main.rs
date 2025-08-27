use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    basic_example().await;
}

async fn basic_example() {
    say_hello().await;
}

async fn say_hello() {
    println!("Hello...");
    sleep(Duration::from_secs(1)).await;
    println!("...world!");
}
