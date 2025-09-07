fn main() {
    // Immutable variable (change नहीं कर सकते)
    let x: i32 = 10;
    println!("x = {}", x);

    // Mutable variable (change कर सकते हो)
    let mut y: i32 = 20;
    println!("y (before) = {}", y);
    y = 25;
    println!("y (after) = {}", y);

    // Floating point number
    let pi: f64 = 3.14159;
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
