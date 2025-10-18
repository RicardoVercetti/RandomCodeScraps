// just use documents, no LLMs 

// - lets see what does this ! does in rust


// phase II - lets go full on LLMs

// 1. declarative macro - macro_rules!()

// 2. procedural macro - #[derive]


// simple declarative macro
#[allow(unused_macros)]
macro_rules! say_hello {
    () => {
        println!("hello word!");
    }
}

// macro with values
#[allow(unused_macros)]
macro_rules! greet {
    ($name:expr, $age:expr) => {
        println!("Hello, {}. you are {}!", $name, $age);
        let new = $age + 5;
        println!("age plus 5 is {}", new);
    };
}

#[allow(unused_macros)]
macro_rules! sum {
    ( $( $x:expr ),* ) => {
        0 $( + $x )*
    };
}

#[allow(unused_macros)]
macro_rules! compare {
    ($x:expr, $y:expr) => {
        if $x > $y {
            println!("the {} is grater than {}", $x, $y);
        } else if $y > $x {
            println!("the {} is grater than {}", $y, $x);
        } else {
            println!("{} and {} are equal!", $x, $y);
        }
    }
}

macro_rules! warp {
    ($x:expr) => {
        let val = String::new(format!("{}", $x));
        println!("str version is {}", val);
    }
}

fn main() {
    println!("Nothing happens!");
    // say_hello!();
    greet!("mike", 29);
    let total = sum!(1, 2, 3, 4);
    println!("Total: {}", total);
    compare!(4, 7);
    warp!(2);
    // let mut a: Vec<u32> = Vec::new();
    // a.push(1);
    // a.push(3);
    // a.push(5);
    // a.push(8);
    // println!("here is a value: {:?}", a);
}
