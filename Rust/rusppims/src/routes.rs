use axum::{
    Json,
};
use serde::{Deserialize, Serialize};

// simple pings

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

// add customer
#[derive(Serialize, Deserialize, Debug)]
struct AddCustomer {
    ref_id: String,
    unique_id: String,
    customer_id: String,
    customer_name: String,
    maiden_name: String,
    date_of_birth: String,
    email_id: String,
    address_line1: String,
    address_line2: String,
    city: String,
    state: String,
    pincode: String,
    account_number: String,
    account_status: String,
    card_number: String,
    card_exp_date: String,
    card_status: String,
    kyc_flag: String,
    kyc_updated_channel: String,
    kyc_updated_on: String,
    system_id: String,
    ovid_value: String,
    ovid_type: String,
    cif_id: String,
    customer_type: String,
    ppi_type: String,
    nri_flag: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    add_customer: AddCustomer,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Risk;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCustomerRequst {
    data: Data,
    // risk: Risk,
}

pub async fn add_customer_handler(Json(payload): Json<AddCustomerRequst>) -> String {
    // request should be successfully received
    println!("req: {:?}", payload);

    "Simple success response from Rusppims".to_string()
}