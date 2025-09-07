fn main() {
    let a1 = String::from("aditya");
    let a2 = take_ownership(a1);
    println!("number is {}", a2);
}
fn take_ownership(s: String) -> String {
    println!("after the ownership is: {}", s);
    s
}
