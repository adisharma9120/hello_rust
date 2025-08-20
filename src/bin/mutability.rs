// fn main(){
//     let mut x = String::from("hi am rich");
    
//     // u try to push something 
//     x.push_str(" always");
  
//   println!("{}",x);
// }

// Mutable refrence in  borrowing and reference..

// fn main()
// {
//     let  mut s1 = String::from("hloo");
//     update_str(&  mut s1); 
//     print!("{}",s1);
// }
//  fn update_str(s:& mut String)
//  {
//     s.push_str(" world"); //cannot borrow `*s` as mutable, as it is behind a `&` reference
// // `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
//  } 
fn main() {
    let mut s1 = String::from("hello");

    {   // scope 1
        let s2 = &mut s1;
        s2.push_str(" world");
        println!("mutable ref: {}", s2);
    } // yaha s2 ka scope khatam ho gaya

    {   // scope 2
        let s3 = &s1;
        println!("immutable ref: {}", s3);
    }

    println!("final s1: {}", s1);
}
