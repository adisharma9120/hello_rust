struct User {
    name: String,
    age: u32,
}

fn main() {
    let name = String::from("aditya");
    let user = User { name, age: 32 };
    println!("{} is {} year old.", user.name, user.age)
}
