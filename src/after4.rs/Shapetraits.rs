/* trait Shape {
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
 */

trait Shape {
    fn area(&self) -> f64;
}
struct Rectangle {
    width: f64,
    heigth: f64,
}
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.heigth
    }
}
fn print_area(shape: &impl Shape) {
    println!("Area is :{}", shape.area());
}

fn main() {
    let rectangle = Rectangle {
        width: 4.0,
        heigth: 4.0,
    };
    print_area(&rectangle);
}
