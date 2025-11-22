use axum::{
    routing::{get, post},
    Json,
    Router,
};
use serde::{Deserialize, Serialize};

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
    let app = Router::new().route("/", get(|| async { "PPIMS here, howdy partner!" }).post(ping_post));

    // run our app with hyper, listening globally on port 3000
    let port = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(port).await.unwrap();
    println!("Running http server on :{port}");
    axum::serve(listener, app).await.unwrap();
    
}

async fn ping_post(Json(payload): Json<PingPostData>) -> Json<PingPostResponse> {
    println!("req: {:?}", payload);


    Json(PingPostResponse { message: "Requst received successfully".to_string() })
}