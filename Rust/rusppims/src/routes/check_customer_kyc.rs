use std::sync::Arc;

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{store::CustomerInfo};
use crate::utils::print_req_res;

pub async fn handle_check_customer_kyc(
    State(_state): State<Arc<RwLock<Vec<CustomerInfo>>>>,
    Json(payload): Json<CheckKycRequest>) -> Json<CheckKycResponse> {

        print_req_res(&payload, "req");


        let res = CheckKycResponse::new(&"500".to_string(), &"NA".to_string(), &"NA".to_string(), &"NA".to_string(), &"NA".to_string(), &"NA".to_string());

        print_req_res(&res, "res");
        Json(res)
    }

// DTOs

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckKycRequest {
    #[serde(rename="Data")]
    pub data: CheckKycRequestData,

    #[serde(rename="Risk")]
    pub risk: Risk
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckKycRequestData {
    #[serde(rename="Check_KYC")]
    pub check_kyc: CheckKyc,

    #[serde(rename="Username")]
    pub username: String,

    #[serde(rename="Password")]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckKyc {
    #[serde(rename="Ref_Id")]
    pub ref_id: String,

    #[serde(rename="Unique_Id")]
    pub unique_id: String,

    #[serde(rename="System_Id")]
    pub system_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Risk {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckKycResponse {
    #[serde(rename="Data")]
    pub data: CheckKycResponseData,

    #[serde(rename="Risk")]
    pub risk: Risk,

    #[serde(rename="Links")]
    pub links: Links,

    #[serde(rename="Meta")]
    pub meta: Meta
}

impl CheckKycResponse {
    fn new(resp_code: &String, kyc_flag: &String,kyc_update_channel: &String, kyc_updated_on: &String, cif_id: &String, unique_id: &String) -> Self {
        CheckKycResponse {
            data: CheckKycResponseData {
                resp_code: resp_code.clone(), 
                kyc_flag: kyc_flag.clone(), 
                kyc_updated_channel: kyc_update_channel.clone(), 
                kyc_updated_on: kyc_updated_on.clone(), 
                cif_id: cif_id.clone(), 
                unique_id: unique_id.clone(),
                old_unique_id: String::from("NA") 
            },
            risk: Risk {},
            links: Links {},
            meta: Meta {},
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckKycResponseData {
    #[serde(rename="Resp_Code")]
    pub resp_code: String,

    #[serde(rename="KYC_Flag")]
    pub kyc_flag: String,

    #[serde(rename="KYC_Updated_Channel")]
    pub kyc_updated_channel: String,

    #[serde(rename="KYC_Updated_On")]
    pub kyc_updated_on: String,

    #[serde(rename="Cif_Id")]
    pub cif_id: String,

    #[serde(rename="Unique_Id")]
    pub unique_id: String,

    #[serde(rename="Old_Unique_Id")]
    pub old_unique_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {}