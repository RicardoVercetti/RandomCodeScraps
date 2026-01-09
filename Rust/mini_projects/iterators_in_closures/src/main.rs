#[derive(Debug)]
struct MyType {
    name: String,
    typ: Typ
}

#[derive(Debug)]
enum Typ {
    Class,
    Function
}

fn main() {
    println!("Series of items with iterators...");

    let tpe = vec![
        MyType { name: "type: 1".to_owned(), typ: Typ::Class },
        MyType { name: "type: 2".to_owned(), typ: Typ::Function },
        MyType { name: "type: 3".to_owned(), typ: Typ::Class }
        ];

    // println!("{:#?}", tpe);
    let mut iter = tpe.iter();

    let mut n = 0usize;
    let len = tpe.len();

    while n < len + 1 {
        let item = iter.next();
        println!("{:?}", item);
        n+=1;
    }
}
