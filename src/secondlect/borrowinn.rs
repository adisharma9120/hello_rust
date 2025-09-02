fn main() {
    let s1 = String::from("Aditya");

    borrow_string(&s1);
    println!("function ke  bad bhi string use ho skti hai: {}", s1);

    let length = calculate_length(&s1);
    println!("'{}' ki  value hai: {}", s1, length);
}

fn borrow_string(s: &String) {
    println!("Borrow String : {}", s);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
