// todo:
// 1. download the file and save with extension
// 2. download checksum and save with extension
// 3. saving as a temp file?
// 4. saving in a temp file with lock on?
// 5. processing the file while the download is on(like the inline doc says in prek)?
// 6. use sha256 to calculate the hash and [u8] → hex string
// 7. try a closure function that will have different methods to extract content from the checksum URL

use futures::TryStreamExt;
use tokio_util::compat::FuturesAsyncReadCompatExt;


fn get_filename_from_url(full_url: &String) -> Option<String> {
    // get the string after the last '/'
    let pos = full_url.rfind("/");
    if let Some(position_value) = pos {
        let subbed = full_url.get((position_value + 1)..full_url.len()).expect("sub-stringing failed!");

        // find the first '.' and return all the strings after that
        let op_ext = subbed.find(".");

        if let Some(ext_pos) = op_ext {
            let opt_ext = subbed.get(ext_pos..subbed.len());
            if let Some(ext) = opt_ext {
                return Some(ext.to_string());
            }
        }
    }
    None
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello omniverse...");

    let full_url = String::from("https://releases.astral.sh/github/uv/releases/download/0.11.17/uv-i686-unknown-linux-gnu.tar.gz");

    let filename = get_filename_from_url(&full_url);

    if let Some(fname) = filename {
        println!("filename is: {}", fname);
    } else {
        println!("Function did not return a filename");
    }

    let response = reqwest::get(&full_url)
    .await?;

    let _response_headers = response.headers().clone();      // if the file name is not found from the URL, gotta rely on the names in the header
    // let _response_bytes = response.bytes().await?;

    let tarball = response
        .bytes_stream()
        .map_err(std::io::Error::other)
        .into_async_read()
        .compat();

    





    Ok(())
}
