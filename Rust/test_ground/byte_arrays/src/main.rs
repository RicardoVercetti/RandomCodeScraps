// concatinate two [u8] arrays


fn main() {
    println!("byte arrays in rust...");
    let arr: Vec<u8>= Vec::from([1, 2, 3, 4, 5, 6, 6, 7, 8, 9, 10]);
    println!("full: {:?}", arr);
    let (arr1, arr2) = arr.split_at(3);
    println!("arr1: {:?}", arr1);
    println!("arr2: {:?}", arr2);
    let mut concatinated = Vec::with_capacity(arr1.len() + arr2.len());
    concatinated.extend_from_slice(arr1);
    concatinated.extend_from_slice(arr2);
    println!("final: {:?}", concatinated);

    let header = vec![0u8; 4];
    println!("header: {:?}", header);

    // let length_int = arr1.len() + arr2.len();
    println!("len int: {}", 255);
    println!("{:04X}", 255);             // 0-15, 16-255,      256-4095, 4096-65535
}

// fn prepare_header_bytes(size: i32) {
//     if size < 16 {

//     }
// }
