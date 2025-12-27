use std::sync::Arc;

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{store::CustomerInfo, utils::print_req_res};
pub async fn handle_update_customer_limit(
    State(_state): State<Arc<RwLock<Vec<CustomerInfo>>>>,
    Json(payload): Json<UpdateCustomerLimitRequest>
) -> Json<UpdateCustomerLimitResponse> {
    
    print_req_res(&payload, "Req");

    let res = UpdateCustomerLimitResponse::new("500".to_string(), "NA".to_string());

    print_req_res(&res, "Res");
    Json(res)
}

// DTOs
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCustomerLimitRequest {
    #[serde(rename="Data")]
    pub data: UpdateCustomerLimitRequestData,

    #[serde(rename="Risk")]
    pub risk: Risk,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCustomerLimitRequestData {
    #[serde(rename="Update_Limit")]
    pub update_customer: UpdateLimit,

    #[serde(rename="Username")]
    pub username: String,

    #[serde(rename="Password")]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateLimit {
    #[serde(rename="Ref_Id")]
    pub ref_id: String,

    #[serde(rename="Unique_Id")]
    pub unique_id: String,

    #[serde(rename="Amount")]
    pub amount: String,
    
    #[serde(rename="Tran_Status")]
    pub tran_status: String,

    #[serde(rename="Tran_Type")]
    pub tran_type: String,

    #[serde(rename="Avail_Bal")]
    pub avail_bal: String,

    #[serde(rename="System_Id")]
    pub system_id: String,

    #[serde(rename="Enquiry_Ref_Id")]
    pub enquiry_ref_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Risk {}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCustomerLimitResponse {
    #[serde(rename="Data")]
    pub data: UpdateCustomerLimitResponseData,
    #[serde(rename="Risk")]
    pub risk: Risk,
    #[serde(rename="Links")]
    pub links: Links,
    #[serde(rename="Meta")]
    pub meta: Meta
}

impl UpdateCustomerLimitResponse {
    fn new(resp_code: String, unique_id: String) -> Self {
        UpdateCustomerLimitResponse {
            data: UpdateCustomerLimitResponseData { resp_code: resp_code, unique_id: unique_id, old_unique_id: "NA".to_string() },
            risk: Risk {  },
            links: Links {  },
            meta: Meta {  }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCustomerLimitResponseData {
    pub resp_code: String,
    pub unique_id: String,
    pub old_unique_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {}