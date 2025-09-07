fn main() {
    // Ownership moves into the function and then comes back
    let s1 = String::from("Rust is powerful");
    let s2 = take_and_return(s1);
    println!("Got it back: {}", s2);

    // Borrowing (no ownership transfer)
    let s3 = String::from("Borrowing example");
    print_length(&s3);
    println!("s3 is still valid: {}", s3);

    // Mutable borrow
    let mut s4 = String::from("Hello");
    add_world(&mut s4);
    println!("After change: {}", s4);

    // Lifetimes example
    let s5 = String::from("Aditya");
    let result;
    {
        let s6 = String::from("Sharma");
        result = longest(&s5, &s6);
    }
    println!("Longest string is: {}", result);
}

// Function that takes ownership and returns it back
fn take_and_return(s: String) -> String {
    println!("Ownership taken: {}", s);
    s // return ownership
}

// Borrowing (read-only)
fn print_length(s: &String) {
    println!("Length is: {}", s.len());
}

// Mutable borrow
fn add_world(s: &mut String) {
    s.push_str(" World!");
}

// Lifetime example (both params must live long enough)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
