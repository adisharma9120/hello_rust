use std::collections::HashMap;

fn main() {
    let mut students = HashMap::new();

    students.insert(String::from("Amit"), 'A');
    students.insert(String::from("Sita"), 'B');

    let amit_grade = students.get("Amit");
    println!("Amit's grade: {:?}", amit_grade); // Some('A')
}
