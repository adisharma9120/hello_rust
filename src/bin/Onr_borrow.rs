fn main() {
    // Ownership example
    let s1 = String::from("Hello Rust");

    let s2 = s1; // ownership moves to s2
                 // println!("{}", s1); // ❌ error: s1 no longer valid

    println!("Now s2 owns the string: {}", s2);

    // Borrowing example
    let s3 = String::from("Borrow me!");
    print_string(&s3); // pass reference instead of moving
    println!("s3 is still valid here: {}", s3);

    // Mutable Borrowing example
    let mut s4 = String::from("Hi");
    change_string(&mut s4);
    println!("After change: {}", s4);
}

// Borrowing (reference) - doesn't take ownership
fn print_string(s: &String) {
    println!("Borrowed string: {}", s);
}

// Mutable Borrowing - allows modification
fn change_string(s: &mut String) {
    s.push_str(", Rust!");
}
