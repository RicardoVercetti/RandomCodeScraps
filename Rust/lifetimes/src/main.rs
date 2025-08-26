
struct Highlight<'a> {
    text1: &'a str,
    text2: &'a str,
}

fn longest_word<'a>(words: &'a[&str]) -> &'a str {
    let mut max_word = 0;
    let mut d_word: &str = &"";
    for &word in words {
        if word.len() > max_word {
            max_word = word.len();
            d_word = &word;
        }
    }

    d_word
}

fn main() {

    let sentence = String::from("Rust is fearless and fast");
    let words: Vec<&str> = sentence.split_whitespace().collect();

    println!("Longest word : {}", longest_word(&words));
    
    
    //let word = String::from("oneword");
    //let word2 = String::from("twoword");
    //let val = Highlight { text1: &word, text2: &word2 };
    
    //{    
    //    println!("txt1 : {}", val.text1);
    //    println!("txt2 : {}", val.text2);
    //}
    
    //println!("{} | {}", val.text1, val.text2);
    
}
