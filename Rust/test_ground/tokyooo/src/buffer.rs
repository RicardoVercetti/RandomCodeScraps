use std::io::Error;
use std::io::Read;
use tokio_util::compat::TokioAsyncReadCompatExt;


#[allow(dead_code)]
fn reading_from_buffers() -> Result<(), Error> {
    println!("Hello omniverse...");

    let mut sample_str = "this is a test string".as_bytes();
    println!("value in string before: {:?}", sample_str.to_ascii_lowercase());
    

    let mut buffer = [0; 5];

    loop {
        match sample_str.read(&mut buffer) {
            Ok(0) => break,
            Ok(value) => {
                // some value is read
                // println!("len read: {}", value);
                println!("str: {}", str::from_utf8(&buffer[0..value]).expect("UTF-8 String error"));
            },
            Err(err) => {
                println!("Error occurred while reading from the bytes: {}", err);
                let ret = err.kind();
                println!("{}", ret.to_string());
            }
        }
    }
    Ok(())
}

