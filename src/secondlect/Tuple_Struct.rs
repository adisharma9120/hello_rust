struct Point(i32, i32);

fn main() {
    let p = Point(10, 20);
    println!("Point coordinates: ({}, {})", p.0, p.1);
}
