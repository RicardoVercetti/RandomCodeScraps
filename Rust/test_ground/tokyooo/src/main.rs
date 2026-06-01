// todo:
// 1. download the file and save with extension
// 2. download checksum and save with extension
// 3. saving as a temp file?
// 4. saving in a temp file with lock on?
// 5. processing the file while the download is on(like the inline doc says in prek)?
// 6. use sha256 to calculate the hash and [u8] → hex string
// 7. try a closure function that will have different methods to extract content from the checksum URL


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello omniverse...");

    let response = reqwest::get("https://releases.astral.sh/github/uv/releases/download/0.11.17/uv-i686-unknown-linux-gnu.tar.gz")
    .await?;

    let response_headers = response.headers().clone();
    let _response_bytes = response.bytes().await?;

    // let response_str: String = response_bytes.iter().map(|b| format!("{:02x}", b)).collect();
    
    // println!("response bytes: {}", response_str);
    for (header_name, header_value) in response_headers {
        println!("headerName: {:?}, headerValue: {:?}", header_name.unwrap(), header_value);
    }


    Ok(())
}
