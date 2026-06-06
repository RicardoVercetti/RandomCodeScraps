// todo:
// 1. download the file and save with extension
// 2. download checksum and save with extension
// 3. saving as a temp file?
// 4. saving in a temp file with lock on?
// 5. processing the file while the download is on(like the inline doc says in prek)?
// 6. use sha256 to calculate the hash and [u8] → hex string
// 7. try a closure function that will have different methods to extract content from the checksum URL
// 8. working with buffers

use std::path::Path;
use std::io::Error;
use std::io::Read;

use tokio::io::AsyncReadExt;
// use futures::TryStreamExt;
use tokio_util::compat::{FuturesAsyncReadCompatExt};
use tokio::fs::File;
// use 
// use std::io::prelude::*;


use futures::{
    AsyncWriteExt, 
    TryStreamExt
};

#[allow(dead_code)]
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



#[allow(dead_code)]
async fn async_function() {
    println!("Inside async function");
    let full_url = String::from("https://releases.astral.sh/github/uv/releases/download/0.11.17/uv-i686-unknown-linux-gnu.tar.gz.sha256");

    let filename = get_filename_from_url(&full_url);

    if let Some(fname) = &filename {
        println!("filename is: {}", fname);
    } else {
        println!("Function did not return a filename");
    }

    let response = reqwest::get(&full_url)
    .await.expect("some error occurred inside...");

    let _response_headers = response.headers().clone();      // if the file name is not found from the URL, gotta rely on the names in the header
    // let _response_bytes = response.bytes().await?;

    let tarball = response
        .bytes_stream()
        .map_err(std::io::Error::other)
        .into_async_read()
        .compat();


    // read from the tarbar and write it into a file
    let owned_file_name = filename.or_else(|| Some("Default_value".to_string())).unwrap();
    let path = Path::new(&owned_file_name);
    let file_to_write = File::create_new(path).await.expect("failed to create file");
    
    let mut reader = tokio::io::BufReader::with_capacity(20, tarball);
    let mut writer = tokio::io::BufWriter::with_capacity(20, file_to_write);

    // let mut buffer= &[0; 20];

    // loop {
    //     match reader.read_buf(&mut buffer) {
    //         Ok(0) => {
    //             println!("reached end of stream!");
    //             break;
    //         },
    //         Ok(n) => {
    //             let ret = writer.write(&buffer[0..n]).await;
    //         }
    //     }
    // }

    match tokio::io::copy(&mut reader, &mut writer).await {
        Ok(value) => println!("some value returned when writing the buffer into the writer: {}", value),
        Err(err) => {
            println!("error occurred when writing the buffer to the writer");
            println!("error kind: {}", err.kind())
        }
    }

}


#[tokio::main]
async 
fn main()
-> Result<(), Box<dyn std::error::Error>> 
{
    println!("Welcome...");
    let ret = tokio::spawn(async_function());
    ret.await?;

    Ok(())
}
