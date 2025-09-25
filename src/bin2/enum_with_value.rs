#[derive(Debug)]
enum Shape {
    Circle(f64),

    Rectangle(f64, f64),
}

//function to calculate area based on the shape
fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,

        Shape::Rectangle(w, h) => w * h,
    }
}
fn main() {
    let circle = Shape::Circle(5.0);

    let rectangle = Shape::Rectangle(3.0, 6.0);

    println!("Circle area: {}", calculate_area(circle));

    println!("Rectangle area: {}", calculate_area(rectangle));
}
