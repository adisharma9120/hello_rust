/* fn main() {
    let s = String::from("Rust lang");
    print_string(s); // ownership function ko chala gaya

    // println!("{}", s); // ❌ ye error dega
}

fn print_string(text: String) {
    println!("Inside function: {}", text);
}
 */

 // Returning Ownership from Function

fn take_ownership(s: String) {
    println!("Function ke andar mila: {}", s);
    // yaha se nikalte hi s delete ho jayega (ownership function ke andar khatam)
}

fn main() {
    let name = String::from("Aditya");
    take_ownership(name);

    // println!("{}", name); // ❌ error: name ab valid nahi hai
}
