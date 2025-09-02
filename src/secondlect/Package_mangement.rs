/* /* use chrono::Local;

fn main()
{
 let now = Local::now();
 println!("Current time is {}", now);

}
 */

fn main() {
    let mut name = String::from("aditya");
    name.push_str(" sharma");
    println! {"Full name is: { }", name};
}
 */

 fn main() {
    let first = String::from("aditya");
    let last = String::from("sharma");

    let full_name = first + " " + &last; // `+` operator lagaya aur space add kiya
    println!("Full name is: {}", full_name);
}
