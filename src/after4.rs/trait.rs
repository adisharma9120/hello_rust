trait Speak {
    fn speak(&self) -> String;
}
struct Dog {
    name: String,
}
struct Cat {
    name: String,
}

impl Speak for Dog {
    fn speak(&self) -> String {
        format!("woof i am {}", self.name)
    }
}
impl Speak for Cat {
    fn speak(&self) -> String {
        format!("Meow! i am {}", self.name)
    }
}
fn animal_speak(animal: &impl Speak) {
    println!("{}", animal.speak())
}

fn main() {
    let dog = Dog {
        name: String::from("lucy"),
    };
    let cat = Cat {
        name: String::from("tomy"),
    };

    animal_speak(&dog);
    animal_speak(&cat);
}
