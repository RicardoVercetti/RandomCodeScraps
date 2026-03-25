use std::borrow::Cow;

fn main() {
    println!("trying out kini-itha stuff...");
    // std_borrow_cow();
    // common_borrowing();
    // valid_utf_8();
    str_split_once();
}

fn str_split_once() {
    let log_line = "error:network_timeout:request_failed";

    // slice once
    // let sliced = log_line.split_once(":");
    // match sliced {
    //     Some((first_half, second_half)) => println!("sliced first half: '{}', sliced second half: '{}'", first_half, second_half),
    //     None => println!("no delimiter found"),
    // }

    // split multiple
    let split_multi = log_line.split(":");
    for item in split_multi.enumerate() {
        println!("split value: {}.'{}'", item.0 + 1, item.1);
    }

}

fn valid_utf_8() {
    // converts a [u8] to utf-8 string of &str

    // let bytes = &[43, 46, 66, 67];
    let st = String::from("Hello rust");
    let arr = st.as_bytes();

    println!("array: {:?}", arr);

    // let converted = std::str::from_utf8(arr);

    // match converted {
    //     Ok(value) => println!("conversion successful: '{}'", value),
    //     Err(e) => println!("error occurred: {}", e),
    // }

    // let invalid_bytes = [0xFF, 0x35, 0xDF, 0xFF];
    // println!("values in array: {:?}", invalid_bytes);
    // let decoded = std::str::from_utf8(&invalid_bytes);
    // match decoded {
    //     Ok(value) => println!("value string: {}", value),
    //     Err(e) => println!("error occured: {e}"),
    // }
}

fn common_borrowing() {
    println!("here is some common borrowing...");

    let a = 20;
    let b = &a;

    println!("address of a: {:b}", &a);
    println!("address of b: {:b}", &b);
}

fn std_borrow_cow() {
    // ventures: 
    // uses Borrow trait
    // implements Deref trait

    // other items:
    // reference counting pointers - Rc::make_mut, Arc::make_mut
    println!("Cow sample....");
    
    // CH1: No clone occurs cuz no need to be mutated
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice);
    abs_all(&mut input);

    println!("from slice: {:?}", slice);
    println!("from input: {:?}", input);
    println!("ptr of slice: {:?}", slice.as_ptr());
    println!("ptr of input: {:?}", input.as_ptr());

    // CH2: Clone occurs cuz need to be mutated
    let slice2 = [-1, 2, 3, 4, -5];
    let mut input2 = Cow::from(&slice2);
    abs_all(&mut input2);

    println!("from slice2: {:?}", slice2);
    println!("from input2: {:?}", input2);
    println!("ptr of slice2: {:?}", slice2.as_ptr());
    println!("ptr fo input2: {:?}", input2.as_ptr());

    // CH3: No clone occurs cuz 'input' is already owned
    let mut input3 = Cow::from(vec![-1, 0, 1]);
    
    println!("address before: {:p}", input3.as_ptr());          // always use {:P} for pointers 
    abs_all(&mut input3);
    println!("address after: {:p}", input3.as_ptr());
}

fn abs_all(input: &mut Cow<'_, [i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            input.to_mut()[i] = -v;
        }
    }
}
