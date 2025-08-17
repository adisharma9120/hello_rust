/* fn main() {
    let x: i32 = -5;
    let y: i32 = 1000;
    let z: f64 = 3.14;

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
}
 */
/* 
 fn main() {
    let mut x: i32 = 10; // mutable because we are changing x

    for _ in 0..1000 {   // use `_` if the loop variable is unused
        x = x + 100;     // increase x by 100 each time
    }

    println!("x = {}", x); // println! for newline after printing
}
 
 


fn main(){
    let is_male: bool = true;
    let  mut is_above_18: bool = true;

    is_above_18 = false;
    if is_male{
        print!("you are a male");
    }
    else{
        print!("you are not a male");
    }

    if is_male && is_above_18 {
        print!("you are a legal man");
    }
}
 
*/


// STRING
/* 
fn main(){
    let greeting: String = String::from("hello world");
    println!("{}" , greeting);

//    let mut name: String = String::from("John");


    let char1  = greeting.chars().nth(0);
    println!("char1: {}",char1.unwrap());

}

same code if i run in js so how it looks
function main(){
    let greeting = "hello world";
    console.log(greeting);
    console.log(greeting[0]);
}
main(); */
fn main(){
    let n: i32 = 12;
     
     for i in 0..10
     {
    println!("{}x{}={}",n,i,n * i);
     }
}