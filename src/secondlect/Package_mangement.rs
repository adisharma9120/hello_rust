fn main() {
    println!("--- Example 1: Ownership Move ---");
    let s1 = String::from("hello");
    let s2 = s1; // ownership s1 se s2 me move ho gaya
    println!("s2 = {}", s2);
    // println!("s1 = {}", s1); // ❌ error hota agar uncomment kare

    println!("\n--- Example 2: Ownership with Function ---");
    let s3 = String::from("rust");
    takes_ownership(s3); // ownership transfer ho gaya
    // println!("{}", s3); // ❌ error hota agar uncomment kare

    println!("\n--- Example 3: Borrowing (Immutable Reference) ---");
    let s4 = String::from("borrow");
    borrow_string(&s4); // ownership gaya nahi, sirf borrow hua
    println!("s4 is still available: {}", s4);

    println!("\n--- Example 4: Mutable Borrow ---");
    let mut s5 = String::from("change");
    change_string(&mut s5); // mutable borrow
    println!("s5 after change: {}", s5);
}

fn takes_ownership(some_string: String) {
    println!("Function got ownership of: {}", some_string);
}

fn borrow_string(some_string: &String) {
    println!("Function borrowed: {}", some_string);
}

fn change_string(some_string: &mut String) {
    some_string.push_str("d with borrow");
}
