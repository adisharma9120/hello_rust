#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

//function to calculate area based on the shape
fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Square(s) => s * s,
        Shape::Rectangle(w, h) => w * h,
    }
}
fn main() {
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    println!("Circle area: {}", calculate_area(circle));
    println!("Square area: {}", calculate_area(square));
    println!("Rectangle area: {}", calculate_area(rectangle));
}
