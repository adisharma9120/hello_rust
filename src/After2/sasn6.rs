use std::collections::HashMap;

fn main() {
    let text = "hello world hello rust world rust rust";
    let mut freq = HashMap::new();

    for word in text.split_whitespace() {
        let count = freq.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Word counts: {:?}", freq);
}
