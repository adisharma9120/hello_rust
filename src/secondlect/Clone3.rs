fn main() {
    let base = String::from("Aditya");
    let phrases = vec![" — keep going", " — learning Rust", " — achieving goals"];

    let mut modified = base.clone();

    for phrase in &phrases {
        modified.push_str(phrase);
    }

    println!("Original: {}", base);
    println!("Modified with multiple phrases: {}", modified);
}
