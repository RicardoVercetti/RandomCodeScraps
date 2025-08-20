
trait Shape {
    fn area(&self) -> f64;
}

struct Circle { radius : f64 }
struct Rectangle { width: f64, height: f64 }
struct Triangle { base: f64, height: f64 }

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

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

fn print_area<T: Shape>(shape: &T) {    // traits and generics together, uses dynamic dispatch(runtime lookup)
    println!("Area = {}", shape.area());
}

//fn print_area_dyn(shape: &dyn Shape) {
//    println!("Area = {}", shape.area());
//}

fn print_all_areas(l: &[Box<dyn Shape>]) {
    for item in l {
        //print_area_dyn(item);
        println!("Area = {}", item.area());
    }
}

fn main() {
    println!("Hello, world!");
    
    let c = Circle { radius: 3.0 };
    let r = Rectangle { width: 4.0, height: 5.0 };
    let t = Triangle { base: 3.0, height: 6.0 };
    
    //print_area(&c);
    //print_area(&r);
    
    let mut l: Vec<Box<dyn Shape>> = Vec::new();
    l.push(Box::new(c));
    l.push(Box::new(r));
    l.push(Box::new(t));
    
    print_all_areas(&l);
    
    
}
