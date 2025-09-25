 pub fn main ()
{
    create_str();
 
    print!("{}",23);
}

 pub fn create_str()
{
let s1 = String::from("hi there");
    print!("{}",s1);
    let s2 = &s1;
    println!("{}",s2);
}