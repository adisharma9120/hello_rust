use std::collections::HashMap;

fn main() {
    let mut grades = HashMap::new();
    grades.insert("Alice", 85);
    grades.insert("Bob", 92);
    grades.insert("Charlie", 78);

    let sum: i32 = grades.values().sum();
    let count = grades.len();
    let avg = sum as f32 / count as f32;

    println!("Average Grade: {:.2}", avg);



}
