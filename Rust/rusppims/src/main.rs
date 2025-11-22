use axum::{
    routing::{get, post},
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Serialize, Deserialize)]
struct MyData {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PingPostData {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct PingPostResponse {
    message: String,
}

#[tokio::main]
async fn main() {
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

async fn ping_get() -> String {
    "Rusppims here, howdy partner!".to_string()
}

async fn ping_post(Json(payload): Json<PingPostData>) -> Json<PingPostResponse> {
    println!("req: {:?}", payload);
    // you can do something with the data ya know!

    Json(PingPostResponse {
        message: "Requst received successfully".to_string(),
    })
}
