use axum::{
    Json,
};
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize)]
// struct MyData {
//     message: String,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct PingPostData {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct PingPostResponse {
    message: String,
}


pub async fn ping_get() -> String {
    "Rusppims here, howdy partner!".to_string()
}

pub async fn ping_post(Json(payload): Json<PingPostData>) -> Json<PingPostResponse> {
    println!("req: {:?}", payload);
    // you can do something with the data ya know!

    Json(PingPostResponse {
        message: "Requst received successfully".to_string(),
    })
}
