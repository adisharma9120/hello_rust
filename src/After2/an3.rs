use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Alice", 10);
    scores.insert("Bob", 20);

    println!("All scores: {:?}", scores);

    if let Some(score) = scores.get("Bob") {
        println!("Bob’s score: {}", score);
    }

    scores.remove("Alice");
    println!("After remove: {:?}", scores);
}
``