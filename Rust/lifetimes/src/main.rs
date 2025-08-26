struct Highlight<'a> {
    text: &'a str,
}

fn largest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    //println!("Heellowww derrr...");

    //let a = "Hall";
    //let b = "Ashton";

    //println!("The largest : {}", largest(&a, &b));

    let novel = String::from("Rust makes memory safe.");
    let h = Highlight { text: &novel };

    println!("{}", h.text);
}
