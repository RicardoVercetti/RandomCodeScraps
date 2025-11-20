// Task

// make vector of different shapes using Box<dyn Shape>
// loop through them and print shape's area and perimeter
// Add a Triangle struct and include it in collection


use trait_objects_n_polymorphism::{ Circle, Rectangle, Triangle, Shape };

fn main() {
    println!("-- Trait Box --");
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle { width: 4.0, height: 6.0 }),
        Box::new(Triangle { a: 2.0, b: 4.0, c: 4.0 }),
    ];
    
    for shape in shapes {
        println!("Area : {}, Perimeter: {}", shape.area(), shape.perimeter());
    }
}
