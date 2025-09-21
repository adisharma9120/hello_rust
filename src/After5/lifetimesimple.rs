// write a unction that takes two string
// as an input adn returens the bigger amongst them

fn main() {
    let a = String::from("aditya");
    let b = String::from("sharma");

    println!("largest string is :{}", largest_string(&a, &b));
    //     println!("largest string is: {}", largest_string(a, b));
}
// fn largest_string(a: &String, b: &String) -> &String {
fn largest_string<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() { a } else { b }
}
