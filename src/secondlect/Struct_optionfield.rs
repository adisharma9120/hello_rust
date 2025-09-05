struct Student {
    name: String,
    grade: Option<char>,
}

fn main() {
    let s1 = Student {
        name: String::from("Ravi"),
        grade: Some('A'),
    };
    let s2 = Student {
        name: String::from("Neha"),
        grade: None,
    };

    match s1.grade {
        Some(g) => println!("{} got grade {}", s1.name, g),
        None => println!("{} has no grade yet", s1.name),
    }

    match s2.grade {
        Some(g) => println!("{} got grade {}", s2.name, g),
        None => println!("{} has no grade yet", s2.name),
    }
}
