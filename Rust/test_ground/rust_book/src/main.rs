use std::{ops::Add, time::Duration};

use time::{
    // Date, 
    Time
};

#[allow(dead_code)]
fn first_word(full_string: &str) -> &str {
    let bytes = full_string.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &full_string[..i];
        }
    }
    full_string
}

#[allow(dead_code)]
fn sub_string_thing() {
    println!("Trying out the slice type...");
    let words = "Here is a string or words for testing";
    let ret_str = first_word(words);
    println!("return string: {}", ret_str);
}

#[allow(dead_code)]
fn some_and_none() {
println!("Some and None values");
    // let some_value = Option::Some("here is some value");
    let some_other_value: Option<String> = Option::None;

    if let Some(value) = some_other_value {
        println!("value: {}", value);
    } else {
        println!("There is no value....");
    }

    // can I use the value variable here?
    // println!("value outside: {}", value);

}

#[cfg(feature = "foo")]
fn some_function() {
    println!("this function does nothing....");

    let value = cfg!(target_os = "ios");
    println!("value for mac: {}", value);

    if cfg!(target_os = "linux") {
        println!("Running in linux BTW...");
    }

}

// #[cfg(unix)]
// fn some_other_function() {
//     println!("This function also does nothing...");
// }

fn main() {
    println!("hay there...");

    let duration = Duration::from_secs(25);

    let time = Time::from_hms(12, 25, 26).unwrap();
    let second_time = time.add(duration);
    println!("{}", time);
    println!("{}", second_time);

    some_function();
    // some_other_function();
}
