// write a unction that takes two string
// as an input adn returens the bigger amongst them

fn main() {
    let a = String::from("aditya");
    let b = String::from("sharma");

    println!("largest string is :{}", largest_string(a,b));
}

fn largest_string(a: String, b: String) -> String {
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
}
