use rand::Rng;

#[derive(Debug)]
enum ShirtColor {
    Blue,
    Green,
    Yellow,
    Orange,
    Pink,
    Mellow,
    Mango
}

#[derive(Debug)]
struct People {
    name: String,
    color: Option<ShirtColor>
}

#[derive(Debug)]
struct ShirtSets {
    color: ShirtColor,
    number: i32
}



fn main() {
    println!("{}", repeat_string("=", 80));
    println!("We're giving away free themed t-shirts, \nregister today to get a chance to with a t-shirt with the color of your choice...");
    println!("{}", repeat_string("=", 80));

    println!("rnd: {}", random_in_range(1, 10));

    let company_stocks = create_random_shirts(50);

    println!("company has: {:#?}", company_stocks);

    let customers = create_registered_customers();

    let res = pick_give_away(&customers, &company_stocks);

    println!("{} won a {:?} T-shirt, Congratulations!", res.0.name, res.1);
}

fn create_registered_customers() -> Vec<People> {
    let mut cus = Vec::new();

    cus.push(People { name: "Manny".to_string(), color: None });
    cus.push(People { name: "Kelsi".to_string(), color: Some(ShirtColor::Green) });
    cus.push(People { name: "Mike".to_string(), color: Some(ShirtColor::Orange) });
    cus.push(People { name: "Mikeal".to_string(), color: Some(ShirtColor::Pink) });
    cus.push(People { name: "Jackie".to_string(), color: None });
    cus.push(People { name: "Perkins".to_string(), color: None });
    cus.push(People { name: "Zuko".to_string(), color: Some(ShirtColor::Blue) });
    cus.push(People { name: "Margrate".to_string(), color: None });

    cus
}

// if a customer has a choice, get that color else get for whats the max of color
fn pick_give_away<'a>(registered_customers: &'a Vec<People>, stock: &'a Vec<ShirtSets>) -> (&'a People, &'a ShirtColor) {
    let customer_chosen = choose_customer(registered_customers);
    println!("winner: {}", customer_chosen.name);

    // if the customer has a choice, we can get that color
    match customer_chosen.color.as_ref() {
        Some(color) => {
            println!("{} has chosen the color {:?}", customer_chosen.name, color);
            (customer_chosen, color)
        },
        None => {
            let auto_choice = get_highest_item_stock(stock);
            println!("{} didn't not choose anything, therefore choosing for them: {:?}", customer_chosen.name, auto_choice);
            (customer_chosen, auto_choice)
        }
    }
}

fn get_highest_item_stock(stock: &Vec<ShirtSets>) -> &ShirtColor {
    let mut max = 0;
    let mut max_color: Option<&ShirtColor> = None;
    for item in stock {
        if item.number > max {
            max = item.number;
            max_color = Some(&item.color);
        }
    }
    max_color.unwrap()
}

fn choose_customer<'a>(registered_customers: &'a Vec<People>) -> &'a People {
    let choice = random_in_range(0, (registered_customers.len() -1) as i32);
    registered_customers.get(choice as usize).unwrap()
}

fn create_random_shirts(max_of: i32) -> Vec<ShirtSets> {
    let mut set = Vec::new();
    set.push(ShirtSets{ color: ShirtColor::Blue, number: random_in_range(1, max_of) });
    set.push(ShirtSets { color:ShirtColor::Green, number: random_in_range(1, max_of) });
    set.push(ShirtSets { color:ShirtColor::Mango, number: random_in_range(1, max_of) });
    set.push(ShirtSets { color:ShirtColor::Mellow, number: random_in_range(1, max_of) });
    set.push(ShirtSets { color:ShirtColor::Orange, number: random_in_range(1, max_of) });
    set.push(ShirtSets { color:ShirtColor::Yellow, number: random_in_range(1, max_of) });

    set
}


fn repeat_string(s: &str, n: i32) -> String {
    let mut new_str = String::new();
    for _ in 1..=n {
        new_str.push_str(s);
    }

    new_str
}

/// from start to finish, both inclusive
fn random_in_range(start: i32, finish: i32) -> i32 {
    let mut ran = rand::thread_rng();
    ran.gen_range(start..=finish)
}