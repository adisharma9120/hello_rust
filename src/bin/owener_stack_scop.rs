// /*
// # Example = 1
// Passimng stack variable onsode function */
// fn  main(){
//     let x = 2;
//     let y = 4;
//      println!("sum is : {}", sum(x,y));
//      print!("hello world");
// }
//  /*    a aur b function ke parameters hote hain.
// Jab tu sum(x, y) call karta hai, tab x ka value copy hokar a me chala jata hai aur y ka value copy hokar b me chala jata hai.
// Isko hum argument pass karna kehte hain.
// Yaani function ke andar ka naam (a, b) alag hai aur bahar se jo values di jaati hain (x, y) alag hain. */
// fn sum(a: i32, b: i32) -> i32{
//     let c = a+b;
//     return c;
//  }

// /*
//  #Example = 2;
//  Scoping variable in the same function; */
//  fn main1 (){
//     let x = 3; // create on stack
//     {
//         let y = 3; // create  on stack
//     }
//     println!("{}",y);  // throw error
//  }

// /*  fn main() {
//     {
//         let y = 10;
//         println!("Inside block y = {}", y); // ✅ y yaha milega
//     }
//     println!("Outside block y = {}", y);   // ❌ Error, kyunki y destroy ho gaya
// } */

// fn main()
// {
//     let my_string= String::from("hello");
//     takes_ownership(my_string.clone());
//     println!("{}", my_string);  // this  line would cause a complier error beacise ownership has been
//     // moved
// }

//  fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
//      // return ownership
// }

fn main() {
    let s1 = String::from("chal be chutiye ");
    let s2 = &s1;

    println!("{}", s2);
    println!("{}", s1) // this is  valid ,The first pointer wasn't invalidated\
}
