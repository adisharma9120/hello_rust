fn main()   
{
    let s1 = String::from("aditya");
    // let s2 = s1;

    do_something(s1.clone());
            println!("number is {}", s1);

    }

    fn do_something(s2: String){
        println!("{}",s2);
}