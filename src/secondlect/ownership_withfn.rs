fn main() {
    let s = String::from("Rust lang");
    print_string(s); // ownership function ko chala gaya

    // println!("{}", s); // ❌ ye error dega
}

fn print_string(text: String) {
    println!("Inside function: {}", text);
}
