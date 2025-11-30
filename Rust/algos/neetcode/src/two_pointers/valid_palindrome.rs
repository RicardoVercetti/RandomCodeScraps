// valid palindrome

// a palindrome is a text that is the same reading from forward(left to right) or backward(right to left).
// Ignore everything other than alphanumeric charecters. And alphabets must be compared as case insensitive
// return true if palindrome, else false

// eg-1: "Was it a car or a cat I saw?"     out: true
// eg-2: "tab a cat"                        out: false

// soln: 
// remove all non-alphanumeric charecters
// make the string into same case(lower or upper case)
// whatever this result string is, have a reversed copy of this
// now compare two strings and return true or false based on the comparison

#[allow(unused)]
fn is_palindrome(s_in_q: &String) -> bool {                         // TODO: this can be optimized using two pointer solution
    let collected: String = s_in_q.chars().into_iter().filter(|a| a.is_alphanumeric() && !a.is_whitespace()).collect();
    let new_str = collected.to_lowercase();
    let rev_str: String = new_str.chars().into_iter().rev().collect();
    // println!("new str: '{}'", new_str);
    // println!("rev str: '{}'", rev_str);
    new_str == rev_str
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        let sample_1 = String::from("Was it a car or a cat I saw?");
        let result = is_palindrome(&sample_1);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let sample_2 = String::from("tab a cat");
        let result = is_palindrome(&sample_2);
        assert_eq!(result, false);
    }
}