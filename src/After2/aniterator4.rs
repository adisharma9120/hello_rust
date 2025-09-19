use std::collections::HashMap;

fn main() {
    let mut capitals = HashMap::new();
    capitals.insert("India", "New Delhi");
    capitals.insert("Japan", "Tokyo");
    capitals.insert("France", "Paris");

    for (country, city) in &capitals {
        println!("The capital of {} is {}", country, city);
    }
}
