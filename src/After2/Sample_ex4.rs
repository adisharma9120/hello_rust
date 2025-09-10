use std::collections::HashMap;

fn main() {
    let mut employees = HashMap::new();

    employees.insert(String::from("Raj"), 50000);
    employees.insert(String::from("Neha"), 65000);

    let raj_salary = employees.get("Raj");
    println!("Raj's salary: {:?} INR", raj_salary); // Some(50000)
}
