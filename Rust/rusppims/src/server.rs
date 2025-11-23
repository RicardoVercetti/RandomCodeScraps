
use axum::{
    routing::{get},
    Router,
};

use crate::routes::{
    ping_get,
    ping_post
};

use tokio::net::TcpListener;


pub async fn start_server() {
    // build our application with a single route
    let app: Router = Router::new().route(
        "/",
        get(ping_get).post(ping_post),
    );

    // run our app with hyper, listening globally on port 3000
    let port: &str = "0.0.0.0:3000";
    let listener: TcpListener = tokio::net::TcpListener::bind(port).await.unwrap();

    let server = axum::serve(listener, app);
    println!("Running http server on :{port}");
    server.await.unwrap();
}

