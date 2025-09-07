fn main() {
    // Printing text
    println!("Hello, Rust!");

    // Variables
    let x = 10; // immutable variable
    let mut y = 20; // mutable variable
    y = y + 5;

    // If-else condition
    if y > x {
        println!("y is greater than x");
    } else {
        println!("x is greater or equal to y");
    }

    // Loop
    for i in 1..5 {
        println!("Number: {}", i);
    }

    // Function call
    greet("Aditya");
}

// Function
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
