/* use std::collections::HashMap;

fn main() {
    let mut user = HashMap::new();

    user.insert(String::from("Aditya"), 22);
    user.insert(String::from("sharma"), 32);

    let _first_user_age = user.get("Aditya");

    match _first_user_age {
        Some(age) => println!("age is {}", age),
        None => println!("user not found in the ab"),
    }
}
 

 Fir match ya if let kyun use karte hain?

Sirf isliye taaki tum raw Some(22) ke bajaye clean aur human-readable output print kar sako, 
aur agar key missing ho to crash na ho
*/
use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();

    users.insert(String::from("harkirat"), 22);
    users.insert(String::from("raman"), 32);

    let first_user_age = users.get("harkiwrat");
    println!("First user age: {:?}", first_user_age); // Some(22)
}





 