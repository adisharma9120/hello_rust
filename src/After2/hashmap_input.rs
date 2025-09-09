use std::collections::HashMap;
use std::io;

fn main(){
    let mut hm: HashMap<String, i32> = HashMap::new();

    let mut name = String::new();
    println!("Enter the name :");

    io::stdin().read_line(&mut name).unwrap();
let name = name.trim().to_string();
hm.insert(name, 23);
println!("{:?}", hm);
}