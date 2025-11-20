//use std::io;

// convert temperatures Fahrenheit and Celcius
// Generate the nth Fibonacci number
// print the lyrics to the Christmas carol "The twelve days of christmas"
// taking advantage of the repetition in the song

fn main() {
    println!("Some more code");
    
    //println!("Enter a number : ");
    
    //let mut inp = String::new();
    //io::stdin().read_line(&mut inp)
      //      .expect("failed to fetch input");
    
    
    //let n: i128 = inp.trim().parse().expect("enter numbr");
    // nth fib
    //println!("The fib of {n} is {}", fib(n));

    christmas_carol();
}

// Ex: 1
#[allow(dead_code)]
fn f_to_c(f: i16) -> f32 {
    ((f - 32) as f32) * (5 as f32 / 9 as f32)
}

// Ex: 2
#[allow(dead_code)]
fn fib(n: i128) -> i128 {
    let mut _prev: i128 = 0;
    let mut last: i128 = 0;
    let mut curr: i128 = 0;
    for num in 1..=n {
        if num == 1 {
            curr += 0;
            last = 0;
        } else if num == 2 {
            curr += 1;
            last = 0;
        } else {
            _prev = last;
            last = curr;
            curr = _prev + last;
        }
    }
    curr
}

fn pr(s: &str) {
    println!("{}", s);
}

fn christmas_carol() {
    let sec_line = "My true love sent to me";
    
    pr("Song: Twelve Days Of Christmas");
    println!();
    // seg 1
    main_line("first");
    pr(sec_line);
    let _1 = "A partridge in a pear tree";
    pr(_1);
    
}

fn main_line(ln: &str) {
    println!("On the {ln} day of Christmas");
}


