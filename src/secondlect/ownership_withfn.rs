/* fn main() {
    let s = String::from("Rust lang");
    print_string(s); // ownership function ko chala gaya

    // println!("{}", s); // ❌ ye error dega
}

fn print_string(text: String) {
    println!("Inside function: {}", text);
}


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

*/

fn give_ownership() -> String {
    let s = String::from("Hello Rust");
    s // ownership return kar diya
}

fn take_and_give_back(s: String) -> String {
    println!("Mila hua string: {}", s);
    s // ownership wapas return kar diya
}

fn main() {
    let s1 = give_ownership();
    // ab s1 ke paas "Hello Rust" ka ownership hai

    let s2 = take_and_give_back(s1);
    // s1 ka ownership gaya function me, function ne wapas return kar diya s2 ko

    println!("Ab mere paas hai: {}", s2); // ✅ ye chalega
}
