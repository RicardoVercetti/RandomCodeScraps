
fn longest_word<'a>(words: &'a [&'a str]) -> &'a str {
    let mut a = 0;
    let mut b: &'a str = &"";
    for word in words {
        if word.len() > a {
            a = word.len();
            b = &word;
        }
    }

    b
}

fn main() {


    let sentence = String::from("Rust is fearless and fast");
    let words: Vec<&str> = sentence.split_whitespace().collect();


    println!("Longest word : {}", longest_word(&words));
}
