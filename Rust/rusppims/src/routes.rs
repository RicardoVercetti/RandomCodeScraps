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
    #[serde(rename="Ref_Id")]
    ref_id: String,

    #[serde(rename="Unique_Id")]
    unique_id: String,

    #[serde(rename="Customer_Id")]
    customer_id: String,

    #[serde(rename="Customer_Name")]
    customer_name: String,

    #[serde(rename="Maiden_Name")]
    maiden_name: String,

    #[serde(rename="Mobile_Number")]
    mobile_number: String,

    #[serde(rename="Date_Of_Birth")]
    date_of_birth: String,

    #[serde(rename="Email_Id")]
    email_id: String,

    #[serde(rename="Address_Line1")]
    address_line1: String,

    #[serde(rename="Address_Line2")]
    address_line2: String,

    #[serde(rename="City")]
    city: String,

    #[serde(rename="State")]
    state: String,

    #[serde(rename="Pincode")]
    pincode: String,

    #[serde(rename="Account_Number")]
    account_number: String,

    #[serde(rename="Account_Status")]
    account_status: String,

    #[serde(rename="Card_Number")]
    card_number: String,

    #[serde(rename="Card_Exp_date")]
    card_exp_date: String,

    #[serde(rename="Card_Status")]
    card_status: String,

    #[serde(rename="KYC_Flag")]
    kyc_flag: String,

    #[serde(rename="KYC_Updated_Channel")]
    kyc_updated_channel: String,

    #[serde(rename="KYC_Updated_On")]
    kyc_updated_on: String,

    #[serde(rename="System_Id")]
    system_id: String,

    #[serde(rename="Ovid_Value")]
    ovid_value: String,

    #[serde(rename="Ovid_Type")]
    ovid_type: String,

    #[serde(rename="Cif_Id")]
    cif_id: String,

    #[serde(rename="Customer_Type")]
    customer_type: String,

    #[serde(rename="Ppi_Type")]
    ppi_type: String,

    #[serde(rename="NRI_Flag")]
    nri_flag: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(rename="Add_Customer")]
    add_customer: AddCustomer,

    #[serde(rename="Username")]
    username: String,

    #[serde(rename="Password")]
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Risk{

}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCustomerRequst {
    #[serde(rename="Data")]
    data: Data,

    #[serde(rename="Risk")]
    risk: Risk,
}

pub async fn add_customer_handler(Json(payload): Json<AddCustomerRequst>) -> String {
    // request should be successfully received
    println!("req: {:?}", payload);

    // TODO: pretty print request response

    // TODO: accept optional params in the requst body, show error response only for non-null values

    // TODO: file storage

    // TODO: dedupe check for if customer exists with the same mobile number

    "Simple success response from Rusppims".to_string()
}