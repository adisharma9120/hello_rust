use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter your name:");

    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim().to_string(); // remove newline

    let mut modified = input.clone();
    modified.push_str(", welcome to Rust!");

    println!("Original: {}", input);
    println!("Modified: {}", modified);
}
