/* fn main() {
    let a1 = String::from("aditya");
    let a2 = take_ownership(a1);
    println!("number is {}", a2);
}
fn take_ownership(s: String) -> String {
    println!("after the ownership is: {}", s);
    s
}
 */

fn main() {
    let a1 = String::from("adithya");
    {
        let a2 = a1.clone(); // if u have 2 owner so no worry create a two space for
                             // storage of data where pointer point the same value

        println!("number is : {}", a2);
    }
    println!("Number is : {}", a1);
}
