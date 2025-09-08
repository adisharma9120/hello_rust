use std::collections::HashMap;

fn main() {
    let mut user = HashMap::new();

    user.insert(String::from("Aditya"), 22);
    user.insert(String::from("sharma"), 32);

    let _first_user_age = user.get("Aditya");
    println!(" First user  age : {:?}", _first_user_age);
}
