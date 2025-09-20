/* fn main() {
    let bigger_float = largest_f64(3.14, 2.72);
    let biggger_bool = largest_bool(true, false);

    println!("bigger_float :{}", bigger_float);
    println!("biggger_bool : {}", biggger_bool);
}

fn largest_f64(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
fn largest_bool(a: bool, b: bool) -> bool {
    if a > b { a } else { b }
}

fn main() {
    let bigger_string = largest_string("apple", "banana");
    println!("Bigger string: {}", bigger_string);
}

fn largest_string<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a > b { a } else { b }
}
 */
fn main() {
    let s1 = String::from("rust");
    let s2 = String::from("programming");

    let bigger = largest_word(&s1, &s2);
    println!("Bigger word is: {}", bigger);
}

fn largest_word<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
