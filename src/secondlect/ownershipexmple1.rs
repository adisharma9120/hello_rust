fn main() {
    // ---- Ownership ----
    let s1 = String::from("Aditya");
    takes_ownership(s1);
    // println!("{}", s1);  // ❌ Error hoga kyunki ownership chali gayi hai

    // ---- Borrowing ----
    let s2 = String::from("Sharma");
    borrow_string(&s2);
    println!("Borrow ke baad bhi use ho raha hai: {}", s2);

    // ---- Mutable Borrowing ----
    let mut s3 = String::from("Hello");
    add_exclamation(&mut s3);
    println!("Mutable borrow ke baad: {}", s3);

    // ---- Ownership Return ----
    let s4 = gives_ownership();
    println!("Function ne ownership wapas di: {}", s4);
}

fn takes_ownership(s: String) {
    println!("Ownership gaya: {}", s);
}

fn borrow_string(s: &String) {
    println!("Borrow hua string: {}", s);
}

fn add_exclamation(s: &mut String) {
    s.push_str("!");
}

fn gives_ownership() -> String {
    let s = String::from("Rusty");
    s
}
