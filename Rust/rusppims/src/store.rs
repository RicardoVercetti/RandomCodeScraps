use tokio::{
    fs::{ 
        File, 
        metadata, 
        read_to_string 
    }, 
    io::AsyncWriteExt
};
use std::io::Error;

// define customer info here

// define add customer here

// define get cusomer info here

// define write to file here

// load file/create file
async fn load_or_create_file() -> Result<String, Error> {
    let filename = "customers.json";
    if metadata(filename).await.is_ok() {
        let content: Result<String, Error> = read_to_string(filename).await;
        return content;
    }

    // file doesn't exist, create one
    println!("customers.json is not found, initiating one...");
    let mut file = File::create(filename).await?;
    file.write_all(b"{}").await?;
    
    return Ok("{}".to_string());
}

// deserialize customer info from the customer.json string


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