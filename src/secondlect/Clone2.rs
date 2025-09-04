fn add_message(original: &str) -> String {
    let mut new_string = original.to_string();
    new_string.push_str(" — keep learning Rust!");
    new_string
}

fn main() {
    let name = String::from("Aditya");
    let modified = add_message(&name);

    println!("Original: {}", name);
    println!("Modified: {}", modified);
}
