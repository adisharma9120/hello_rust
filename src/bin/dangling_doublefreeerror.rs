fn main ()
{
    create_str();
    //garbag collector
    print!("{}",23);
}

fn create_str()
{
let s1 = String::from("hi there");
    print!("{}",s1);
    let s2 = s1;
    println!("{}",s2);
} 

//Example -> 1;

// passing Strings (heap variable ) to function as args

fn main ()
{
    let my_string = String::from("hello");
}