// Tasks
// 1. create project
// 2. Define shape trait with
//   fn area(&self) -> f64
//   fn perimeter(&self) -> f64
// 3. Implement trait for Circle and Triangle
// 4. write a function describe<T: Shape>(shape: &T) that prints both area and perimeter
// 5. in main.rs create Circle and Rectangle, then call describe for both

use shapes_lib::{ Circle, Rectangle, describe };


fn main() {
    println!("Hello, world!");
    
    let c = Circle::new(3.5);
    println!("New object created : {:?}", c);
    //println!("Area of circle : {}", c.area())
    //print_area(&c);
    
    
    let r = Rectangle::new(4.6, 6.7);
    //println!("Area of rect: {}", r.area());
    //print_area(&r);
    
    describe(&c);
    describe(&r);
    
}
