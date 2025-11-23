mod server;
mod routes;

use server::start_server;

#[tokio::main]
async fn main() {
    start_server().await;
}

