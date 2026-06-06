use std::{io::{Write, stdout}, ops::Deref, time::Duration};
use tokio::time::sleep;
use mini_redis::{client, Result};
use tokio::spawn;

#[allow(dead_code)]
async fn say_hello() {
    // println!("Hello Peasant...");
    sleep(Duration::from_millis(100)).await;
    print!("hello, ");
    stdout().flush().unwrap();  // flush so that the above is updated immediately
}

#[allow(dead_code)]
async fn redis_example() -> Result<()> {        // run the server with → `mini-redis-server`
    let mut client = client::connect("127.0.0.1:6379").await?;
    // say_hello().await;

    // set
    client.set("hello", "world".into()).await?;

    // get
    let result = client.get("hello").await?;

    println!("got value from server; result={:?}", result);


    Ok(())
}

#[allow(dead_code)]
async fn say_world() {
    sleep(Duration::from_millis(100)).await;
    println!("world!");
}

#[tokio::main]
async fn main() {       // https://rust-lang.github.io/async-book/
    // say_hello().await;
    // sleep(Duration::from_millis(1000)).await;
    // say_world().await;
    // spawn(say_hello());
    // spawn(say_world());
    // sleep(Duration::from_millis(1000)).await;
    match redis_example().await {
        Ok(_) => println!("performed the action successfully."), 
        Err(err) => println!("error occurred: {:?}", err.deref()),
    }
}
