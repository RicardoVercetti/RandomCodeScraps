
//pub trait Area {
//    fn area(&self) -> f64;
//}

pub trait Shape {
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
}

#[derive(Debug)]
pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

//impl Area for Circle {
//    fn area(&self) -> f64 {
//        std::f64::consts::PI * self.radius * self.radius
//    }
//}

impl Shape for Circle {
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius      
    }
    
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }
}

//impl Area for Rectangle {
//    fn area(&self) -> f64 {
//        self.width * self.height
//    }
//}

impl Shape for Rectangle {
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

//pub fn print_area<T: Area>(shape: &T) {
//    println!("Area = {}", shape.area());
//}

pub fn describe<T: Shape>(spe: &T) {
    println!("Perimeter : {}", spe.perimeter());
    println!("Area : {}", spe.area());
}

