fn main() {
    let name = "Aditya";
    let age = 22;
    let city = "Delhi";

    let profile = format!(
        "Name: {}\nAge: {}\nCity: {}\nMessage: {}",
        name, age, city, "Keep pushing forward with Rust 🚀"
    );

    println!("Profile Information:\n{}", profile);
}
