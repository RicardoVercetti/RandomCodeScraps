use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::routes::{
    ping_get,
    ping_post,
    add_customer_handler
};

use crate::store::{
    deserialize_from_json_string,
    load_or_create_file,
    CustomerInfo
};

use tokio::{net::TcpListener, sync::RwLock};


pub async fn start_server() {
    // load the custome data on startup
    let json_str = load_or_create_file().await.unwrap();
    let customers: Vec<CustomerInfo> = deserialize_from_json_string(&json_str).unwrap();

    // Wrap in Arc<RwLock>
    let shared_state: Arc<RwLock<Vec<CustomerInfo>>> = Arc::new(RwLock::new(customers));

    let app: Router = Router::new()
    .route(
        "/",
        get(ping_get).post(ping_post),
    )
    .route("/axis/non-dmz/api/PPIM/v1/add-customer", post(add_customer_handler))
    // .route("/axis/non-dmz/api/PPIM/v1/check-customer-kyc", method_router)                // check customer kyc
    // .route("/axis/non-dmz/api/PPIM/v1/check-customer-limit", method_router)              // check customer limit
    // .route("/axis/non-dmz/api/PPIM/v1/customer-registration-status", method_router)      // check customer registration status
    // .route("/axis/non-dmz/api/PPIM/v1/update-customer", method_router)                   // update customer
    // .route("/axis/non-dmz/api/PPIM/v1/update-customer-limit", method_router)             // update customer limit
    .with_state(shared_state.clone())
    ;

    // run our app with hyper, listening globally on port 3000
    let port: &str = "0.0.0.0:3000";
    let listener: TcpListener = tokio::net::TcpListener::bind(port).await.unwrap();

    let server = axum::serve(listener, app);
    println!("Running http server on :{port}");
    server.await.unwrap();
}

