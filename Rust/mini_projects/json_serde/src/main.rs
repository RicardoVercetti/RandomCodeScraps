// agenda
// 1. given a json string, capture the key and the value string of the input
// 2. populate it into a struct for easy access
// 3. deserialize, get the json representation from the struct in memory

fn main() {

    let st = "{ \"key\": \"value\", \"key_2\":\"value_2\"}";

    println!("str: {}", st);


    println!("hello form json parser world....");

    parse_json_string(st);
}

// approach
// take a stack datatype
// walk by each value, first one should be segment opener → '{' character,
// the one next can be the 'key' character → this starts with a " and ends with a "
// after this, there must always be a ':' character
// then the value character, this could be a string with double " or a number or another json
// when this one stack of the three values are found, then it could be a comma followed by next character, and/or closing brace → '}'


fn parse_json_string(json_str: &str) {
    // here comes nothing
    // let mut key_value_array: Vec::<(String, String)> = Vec::new();


    // lets not consider the inner json for now
    let mut ptr = 0;
    while ptr < json_str.len() {
        // loop by each element
        let current_item = json_str.get(ptr..ptr+1);
        // println!("character in loop: {}", current_item.unwrap());
        let mut isKey = false;      // true if the current value that's being read is a key
        let mut isValue = false;    // true if the current value that's being read is a value

        match current_item.unwrap() {
            "{" => {
                println!("got: '{}'", current_item.unwrap());
                // start of the array, so continue. First item must be key
                ptr += 1;
                continue;           // isn't there any other way to exit out of the loop?
            }, 
            "\"" => {
                // Approach: have isKey, isValue. Decide based on the previous value
                // if isKey, walk until 
            }
            el => println!("other condition: {}", el),
        }

        ptr += 1;
    }
}

