mod routes;
mod server;
mod store;

use server::start_server;

#[tokio::main]
async fn main() {
    start_server().await;
}
