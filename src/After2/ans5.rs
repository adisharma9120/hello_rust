use std::collections::HashMap;

fn main() {
    let names = vec!["Alice", "Bob", "Charlie"];
    let mut scores = HashMap::new();

    for (i, name) in names.iter().enumerate() {
        scores.insert(*name, (i as i32) * 10);
    }

    println!("Scores: {:?}", scores);
}
