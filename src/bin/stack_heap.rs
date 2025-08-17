fn main()
{
    Stack_fn();
    heap_fn();
    updated_string();
}
fn Stack_fn()
{
    
    let a = 30;
    let b = 32;
    let c = a+b;
    println!("Stack function: The sum of {} and {} is {}", a,b,c);
}
 fn heap_fn()
 {
    let s1 =String::from("hello");
    let s2 = String::from("world");
    let s3 = format!("{} {}",s1,s2);
    println!("Heap function : Combined string is {}",s3);
 }
 fn updated_string()
{
    let mut s = String::from("intail string");
    println!("before upadtes {}",s);
    s.push_str("and some additiona text");
    println!("after updates {}",s);
}