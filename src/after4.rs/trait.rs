trait Speak {
    fn speak(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

// Trait implement for Dog
impl Speak for Dog {
    fn speak(&self) -> String {
        format!("Woof! I am {}", self.name)
    }
}

// Trait implement for Cat
impl Speak for Cat {
    fn speak(&self) -> String {
        format!("Meow! I am {}", self.name)
    }
}

// Function jo kisi bhi Speak trait ko accept kare
fn animal_speak(animal: &impl Speak) {
    println!("{}", animal.speak());
}

// Main function
fn main() {
    let dog = Dog { name: String::from("Lucy") };
    let cat = Cat { name: String::from("Tomy") };

    animal_speak(&dog);
    animal_speak(&cat);
}
