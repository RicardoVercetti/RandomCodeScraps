mod routes;
mod server;
mod store;
mod route_lib;
mod utils;

use server::start_server;

#[tokio::main]
async fn main() {
    start_server().await;
}
