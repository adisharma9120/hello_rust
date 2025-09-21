trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}

fn print_area(shape: &impl Shape) {
    println!("area is : {} ", shape.area());
}

fn main() {
    let circle = Circle { radius: 5.0 };

    print_area(&circle);
}
