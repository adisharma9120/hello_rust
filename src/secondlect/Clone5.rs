use std::io;

fn main() {
    // Take input from user
    let mut input = String::new();
    println!("Enter your name:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let name = input.trim(); // remove newline

    // Create modified messages
    let msg1 = format!("{} welcome to Rust 🚀", name);
    let msg2 = format!("{} keep learning every day 💡", name);
    let msg3 = format!("{} success comes with practice 🔥", name);

    // Print results
    println!("\n--- Personalized Messages ---");
    println!("{}", msg1);
    println!("{}", msg2);
    println!("{}", msg3);
}
