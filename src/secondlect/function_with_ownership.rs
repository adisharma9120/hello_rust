fn main() {
    let s = String::from("hello");
    take_ownership(s);

    let x = 5;
    makes_copy(x);
}

fn take_ownership(s: String) {
    println!("some string is: {}", s);
}
fn makes_copy(x: i32) {
    println!("another copy is: {}", x);
}
