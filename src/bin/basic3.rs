fn main() {
    // Immutable variable (change nahi kar sakte)
    let x: i32 = 10;
    println!("x = {}", x);

    // Mutable variable (change kar sakte ho)
    let mut y: i32 = 20;
    println!("y (before) = {}", y);
    y = 25;
    println!("y (after) = {}", y);

    // Floating point number (best practice: use constant)
    let pi: f64 = std::f64::consts::PI;
    println!("pi = {}", pi);

    // Boolean
    let is_rust_fun: bool = true;
    println!("Is Rust fun? {}", is_rust_fun);

    // Character
    let ch: char = 'A';
    println!("Character is: {}", ch);

    // String slice
    let name: &str = "Aditya";
    println!("Hello, {}", name);
}
