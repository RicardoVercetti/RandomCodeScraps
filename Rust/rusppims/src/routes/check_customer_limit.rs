use std::sync::Arc;

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{store::CustomerInfo, utils::print_req_res};

pub async fn handle_check_customer_limit(
    State(_state): State<Arc<RwLock<Vec<CustomerInfo>>>>,
    Json(payload): Json<CheckCustomerLimitRequest>
) -> Json<CheckCustomerLimitResponse> {
    print_req_res(&payload, "req");

    let res = CheckCustomerLimitResponse::new("500".to_string(), "NA".to_string(), "N".to_string());

    print_req_res(&res, "res");
    Json(res)
}

// DTOs

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckCustomerLimitRequest {
    #[serde(rename="Data")]
    pub data: CheckCustomerLimitRequestData,
    #[serde(rename="Risk")]
    pub risk: Risk
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckCustomerLimitRequestData {
    #[serde(rename="Check_Limit")]
    pub check_limit: CheckLimit,
    #[serde(rename="Username")]
    pub username: String,
    #[serde(rename="Password")]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckLimit {
    #[serde(rename="Ref_Id")]
    pub ref_id: String,
    #[serde(rename="Unique_Id")]
    pub unique_id: String,
    #[serde(rename="Account_Number")]
    pub account_number: String,
    #[serde(rename="Card_Number")]
    pub card_number: String,
    #[serde(rename="Amount")]
    pub amount: String,
    #[serde(rename="Tran_Type")]
    pub tran_type: String,
    #[serde(rename="Avail_Bal")]
    pub avail_bal: String,
    #[serde(rename="System_Id")]
    pub system_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Risk {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckCustomerLimitResponse {
    #[serde(rename="Data")]
    pub data: CheckCustomerLimitResponseData,
    #[serde(rename="Risk")]
    pub risk: Risk,
    #[serde(rename="Links")]
    pub links: Links,
    #[serde(rename="Meta")]
    pub meta: Meta
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckCustomerLimitResponseData {
    #[serde(rename="Resp_Code")]
    pub resp_code: String,
    #[serde(rename="Unique_Id")]
    pub unique_id: String,
    #[serde(rename="Allow_Customer")]
    pub allow_customer: String,
    #[serde(rename="Cumulative_Bal")]
    pub cumulative_bal: String,
    #[serde(rename="Avail_Amount_Limit")]
    pub avail_amount_limit: String,
    #[serde(rename="Cif_Id")]
    pub cif_id: String,
    #[serde(rename="Old_Unique_Id")]
    pub old_unique_id: String,
}

impl CheckCustomerLimitResponse {
    fn new(resp_code: String, unique_id: String, allow_cus: String) -> Self {
        CheckCustomerLimitResponse{
            data: CheckCustomerLimitResponseData {
                resp_code: resp_code,
                unique_id: unique_id,
                allow_customer: allow_cus,
                cumulative_bal: "NA".to_string(),
                avail_amount_limit: "NA".to_string(),
                cif_id: "NA".to_string(),
                old_unique_id: "NA".to_string()
            },
            risk: Risk {}, 
            links: Links {}, 
            meta: Meta {} 
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {}