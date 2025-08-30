enum Shape {
    Rectangle(f64, f64), // width , height
    Circle(f64),         // radius
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rectangle(w, h) => w * h,
            Shape::Circle(r) => 3.14159 * r * r,
        }
    }
}

// <- yeh hamesha bahar hona chahiye
fn main() {
    let rect = Shape::Rectangle(10.0, 20.0);
    let circ = Shape::Circle(5.0);

    println!("Rectangle area: {}", rect.area());
    println!("Circle area: {}", circ.area());
}
