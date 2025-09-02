/* fn main()
{
    let s1 = String::from("hello");
    let s2 = s1;
    println!("String is:{}",s2);
}
 fn create_string(){

    create_string();
 } */

fn create_string() -> String {
    let s = String::from("hello from function");
    s  // ownership return kar rahe hain
}

fn main() {
    // Example 1: Ownership transfer (move)
    let s1 = String::from("hello");
    let s2 = s1; 
    println!("After move, only s2 is valid: {}", s2);

    // Example 2: Clone (deep copy)
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("Using clone, both are valid: {}, {}", s3, s4);

    // Example 3: Function returns ownership
    let s5 = create_string();
    println!("Got from function: {}", s5);

    // Example 4: Borrowing (no move, just reference)
    print_string(&s5);
    println!("After borrowing, still valid in main: {}", s5);
}

fn print_string(s: &String) {
    println!("Function borrowed: {}", s);
}
guit 