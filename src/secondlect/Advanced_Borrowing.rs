fn main() {
    let mut name = String::from("Aditya");

    print_name(&name);

    add_lastname(&mut name);
    println!("after adding last name :{}", name);

    //ownership transfer (Move)
    let owned = take_ownership(name);
    println!("ownnership is now with owned : {}", owned);

    let length = calculate_length(&owned);
    println!("'{}' ki length hai: {}", owned, length);

    // Function returns ownership
    let new_string = give_back_ownership();
    println!("Function ne ownership di: {}", new_string);
}

fn print_name(s: &String) {
    println!("Current name: {}", s);
}

fn add_lastname(s: &mut String) {
    s.push_str(" Sharma");
}

fn take_ownership(s: String) -> String {
    println!("Function ne ownership le li: {}", s);
    s // ownership return kar raha hai
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn give_back_ownership() -> String {
    String::from("New Owned String")
}