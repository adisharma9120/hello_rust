fn main(){
    let mut name = String::from("Aditya");
    name.push_str(" Sharma");
    println!("Name is :{ }", name);

    name.replace_range(8..name.len()," ");
    println!("Name after the replace : {}", name );
}