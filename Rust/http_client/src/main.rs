use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Post {
    userId: u32,
    id: u32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    //println!("Hello, world!");
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let response = reqwest::get(url).await?.json::<Post>().await?;
    
    println!("parsed JSON:\n {:#?}", response);
    Ok(())
}
