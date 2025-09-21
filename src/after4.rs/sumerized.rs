trait Summarized {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summarized for User {
    fn summarize(&self) -> String {
        format!("Name is: {}, and the age is: {}", self.name, self.age)
    }
}

fn main() {
    let user = User {
        name: String::from("Aditya"),
        age: 32,
    };

    println!("{}", user.summarize());
}
