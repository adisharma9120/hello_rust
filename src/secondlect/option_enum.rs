fn main() {
    let index = find_first_a(String::from("aditrya"));

    if index == -1 {
        println!("a not found");
    } else {
        println!("index is {}", index);
    }
}

fn find_first_a(s: String) -> i32 {
    for (index, char) in s.chars().enumerate() {
        if char == 'y' {
            return index as i32;
        }
    }
    return -1; 
}
  