use serde::{
    Deserialize, 
    Serialize
};
use std::io::Error;
use tokio::{
    fs::{File, metadata, read_to_string},
    io::AsyncWriteExt,
};


#[derive(Serialize, Deserialize, Debug)]
struct CustomerInfo {
    unique_id: String, // this is the ppid
    maiden_name: String,
    mobile_number: String,
    date_of_birth: String,
    account_number: String,
    account_status: String,
    card_number: String,
    card_exp_date: String,
    card_status: String,
    kyc_flag: String,
    kyc_updated_channel: String,
    kyc_updated_on: String,
    ovid_value: String,
    ovid_type: String,
    cif_id: String,
}

// TODO: define add customer here

// TODO: define get cusomer info here

// TODO: define write to file here

// TODO: load file/create file
async fn load_or_create_file() -> Result<String, Error> {
    let filename = "customers.json";
    if metadata(filename).await.is_ok() {
        let content: Result<String, Error> = read_to_string(filename).await;
        return content;
    }

    // file doesn't exist, create one
    println!("customers.json is not found, initiating one...");
    let mut file = File::create(filename).await?;
    file.write_all(b"[]").await?;
    
    return Ok("[]".to_string());
}

// deserialize customer info from the customer.json string
fn deserialize_json_string(str: &str) -> Result<Vec<CustomerInfo>, serde_json::Error> {
    let customers_info: Vec<CustomerInfo> = serde_json::from_str(&str)?;
    Ok(customers_info)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;
    use std::path::Path;

    #[tokio::test]
    async fn test_load_or_create_file() {
        // Remove the file if it exists (cleanup before test)
        if Path::new("customers.json").exists() {
            fs::remove_file("customers.json").await.unwrap();
        }

        // Call your function
        let result = load_or_create_file().await.unwrap();

        // Should return "{}"
        assert_eq!(result, "{}");

        // File should now exist
        assert!(Path::new("customers.json").exists());

        // Cleanup after test
        fs::remove_file("customers.json").await.unwrap();
    }
}