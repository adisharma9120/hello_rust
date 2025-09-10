use std::collections::HashMap;

fn main() {
    let mut countries = HashMap::new();

    countries.insert(String::from("India"), String::from("New Delhi"));
    countries.insert(String::from("Japan"), String::from("Tokyo"));

    let capital_of_india = countries.get("India");
    println!("Capital of India: {:?}", capital_of_india); // Some("New Delhi")
}
