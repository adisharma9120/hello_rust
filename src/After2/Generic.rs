fn main() {
    let bigger = largest_i32(1, 2);
    let bigger_char = larger_char('a', 'b');

    println!("{}", bigger);
    println!("{}", bigger_char);
}

fn largest_i32(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn larger_char(a: char, b: char) -> char {
    if a > b { a } else { b }
}
