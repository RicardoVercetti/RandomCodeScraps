
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

pub struct Circle { pub radius: f64 }
pub struct Rectangle { pub width: f64, pub height: f64 }
pub struct Triangle { pub a: f64, pub b: f64, pub c: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.a * self.b * self.c // must be SIN(C)
    }
    
    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}
