fn main() {
    let sentence = String::from("hi this is aditya ");

    let word: &str = &sentence[0..2];
    println!(" word is {}", word);

    let first: &str = &sentence[2..7];
    print!("first word is {}", first);
}
