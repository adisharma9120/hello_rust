fn main() {
    let mut s = String::from("hello");

    // Multiple immutable references at the same time
    let r1 = &s;
    let r2 = &s;
    println!("Immutable borrows: {r1} and {r2}");

    // After r1 and r2 are no longer used, we can create a mutable reference
    let r3 = &mut s;
    r3.push_str(", world");
    println!("Mutable borrow: {r3}");

    // Back to immutable borrow again
    let r4 = &s;
    println!("Final value: {r4}");
}
