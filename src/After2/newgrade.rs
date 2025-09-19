use std::collections::HashMap;

fn main() {
    let mut grades = HashMap::new();

    // Adding grades
    grades.insert("Alice", 85);
    grades.insert("Bob", 92);

    // Adding a new student later
    grades.insert("David", 88);

    println!("Updated Grades: {:?}", grades);
}
