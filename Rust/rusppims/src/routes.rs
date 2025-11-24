use crate::store::{CustomerInfo, generate_ppid, is_customer_exits_by_mobile_number, save_to_file};
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

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

pub async fn ping_post(
    State(state): State<Arc<RwLock<Vec<CustomerInfo>>>>,
    Json(payload): Json<PingPostData>,
) -> Json<PingPostResponse> {
    println!("req: {:?}", payload);
    // you can do something with the data ya know!

    let data: tokio::sync::RwLockReadGuard<'_, Vec<CustomerInfo>> = state.read().await;

    println!("all customer info: {:#?}", data);

    Json(PingPostResponse {
        message: "Requst received successfully".to_string(),
    })
}

// add customer
#[derive(Serialize, Deserialize, Debug)]
pub struct AddCustomer {
    #[serde(rename = "Ref_Id")]
    pub ref_id: String,

    #[serde(rename = "Unique_Id")]
    pub unique_id: String,

    #[serde(rename = "Customer_Id")]
    pub customer_id: String,

    #[serde(rename = "Customer_Name")]
    pub customer_name: String,

    #[serde(rename = "Maiden_Name")]
    pub maiden_name: String,

    #[serde(rename = "Mobile_Number")]
    pub mobile_number: String,

    #[serde(rename = "Date_Of_Birth")]
    pub date_of_birth: String,

    #[serde(rename = "Email_Id")]
    pub email_id: String,

    #[serde(rename = "Address_Line1")]
    pub address_line1: String,

    #[serde(rename = "Address_Line2")]
    pub address_line2: String,

    #[serde(rename = "City")]
    pub city: String,

    #[serde(rename = "State")]
    pub state: String,

    #[serde(rename = "Pincode")]
    pub pincode: String,

    #[serde(rename = "Account_Number")]
    pub account_number: String,

    #[serde(rename = "Account_Status")]
    pub account_status: String,

    #[serde(rename = "Card_Number")]
    pub card_number: String,

    #[serde(rename = "Card_Exp_date")]
    pub card_exp_date: String,

    #[serde(rename = "Card_Status")]
    pub card_status: String,

    #[serde(rename = "KYC_Flag")]
    pub kyc_flag: String,

    #[serde(rename = "KYC_Updated_Channel")]
    pub kyc_updated_channel: String,

    #[serde(rename = "KYC_Updated_On")]
    pub kyc_updated_on: String,

    #[serde(rename = "System_Id")]
    pub system_id: String,

    #[serde(rename = "Ovid_Value")]
    pub ovid_value: String,

    #[serde(rename = "Ovid_Type")]
    pub ovid_type: String,

    #[serde(rename = "Cif_Id")]
    pub cif_id: String,

    #[serde(rename = "Customer_Type")]
    pub customer_type: String,

    #[serde(rename = "Ppi_Type")]
    pub ppi_type: String,

    #[serde(rename = "NRI_Flag")]
    pub nri_flag: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(rename = "Add_Customer")]
    add_customer: AddCustomer,

    #[serde(rename = "Username")]
    username: String,

    #[serde(rename = "Password")]
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Risk {}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCustomerRequst {
    #[serde(rename = "Data")]
    data: Data,

    #[serde(rename = "Risk")]
    risk: Risk,
}

pub async fn add_customer_handler(
    State(state): State<Arc<RwLock<Vec<CustomerInfo>>>>,
    Json(payload): Json<AddCustomerRequst>,
) -> String {
    // request should be successfully received
    println!("req: {:#?}", payload);

    // get customer data and check id
    let customer_data = state.read().await;

    // dedupe check for if customer exists with the same mobile number
    let is_customer_exist = is_customer_exits_by_mobile_number(
        &payload.data.add_customer.mobile_number,
        &customer_data,
    );
    drop(customer_data);
    if !is_customer_exist {
        // must generate PPID and save the same
        let ppid = generate_ppid();
        println!("generated ppid: {}", ppid);

        // map the AddCustomer to CustomerInfo
        let customer_info_map = CustomerInfo::new(&payload.data.add_customer, &ppid);
        let mut customers = state.write().await;

        // add to vec
        customers.push(customer_info_map);

        // async save to file
        let res = save_to_file(&customers).await;
        match res {
            Err(e) => println!("error occured while saving the file: {}", e),
            _ => {}
        }
        return "Customer added successfully".to_string();
    }

    // TODO: response body
    // TODO: use different response code for failed responses
    // TODO: after this use the response code in the response body
    "Customer already exists".to_string()
}
