fn main() {
    let index = find_first_y("aditrya");

    match index {
        Some(i) => println!("index is {}", i),
        None => println!("y not found"),
    }
}

fn find_first_y(s: &str) -> Option<usize> {
    for (index, ch) in s.chars().enumerate() {
        if ch == 'y' {
            return Some(index);
        }
    }
    None
}