struct User {
    active: bool, 
    first_name: String,
    last_name: String,
    age: i32,
}

fn main() {
    let user1 = User {
        active: true,
        first_name: String::from("aditya"),
        last_name: String::from("Sharma"),
        age: 32,
    };
    println!(
        "User : {} {}, age: {}, active : {}",
        user1.first_name, user1.last_name, user1.age, user1.active,
    );
}
