fn main() {
    let mut s1 = String::from("aditya");

    s1 = do_something(s1.clone());   // ✅ function se return value assign karna
    println!("number is {}", s1);
}

fn do_something(s2: String) -> String {   // ✅ return type declare karna
    println!("{}", s2);
    return s2;   // return karna ab valid hai
}
