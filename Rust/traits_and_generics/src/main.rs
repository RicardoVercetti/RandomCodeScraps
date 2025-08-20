
trait Shape {
    fn area(&self) -> f64;
}

struct Circle { radius : f64 }
struct Rectangle { width: f64, height: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn print_area<T: Shape>(shape: &T) {
    println!("Area = {}", shape.area());
}

fn main() {
    println!("Hello, world!");
    
    let c = Circle { radius: 3.0 };
    let r = Rectangle { width: 4.0, height: 5.0 };
    
    print_area(&c);
    print_area(&r);
    
}
