use std::collections::HashMap;

fn main() {
    let mut books = HashMap::new();

    books.insert(String::from("Rust Programming"), 450);
    books.insert(String::from("The Book of Life"), 300);

    let rust_book_price = books.get("Rust Programming");
    println!("Rust book price: {:?}", rust_book_price); // Some(450)
}
