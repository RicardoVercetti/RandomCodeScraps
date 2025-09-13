// play with std rust

fn main() {
    println!("-- Data Types --");
    
    // scalar type
    let val_1 = 98_222;
    let hexx = 0xfd;
    let oct = 0o77;
    let bin = 0b1111_0000;
    let bite = b'A'; // this is u8 only
    
    println!("val_1 : {}", val_1);
    println!("Hex: {}", hexx);
    
    println!("Oct: {}", oct);
    println!("bin: {}", bin);
    println!("bite: {}", bite);
    
    // compound type - tuples & arrays
    let months = ["January", "Feburary", "March", "April", "May", "June",
            "July", "August", "September", "October", "November", "December"];
            
    for month in months {
        println!("{}", month);
    }
    
    println!("{:?}", months);
    
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    
    
}
