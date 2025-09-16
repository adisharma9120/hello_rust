/* use std::string;

/* /* fn main(){
    let number = 21;


 if  number % 2 != 1{
    println!("The number is even");
}else{
  println!("the number is odd")
}

}
fn main()
{
    let is_even = true;
    if is_even
    {
        print!("this is even no");
    }
    else if !is_even
    {
        print!("this is odd no");
    }
}
    */
 // samec code if we are taking the input from the user than what to write let sees

 use std::io;
 fn  main(){
    println!("enter the number"); // as the user for the input
//  create a variable so that store the input
 let mut input = String::new();

// read the input from the user

 io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

// convert the input into integer
let number: i32 = input.trim().parse().expect("please enter the valid number");

// check the number is even or odd
if number % 2 == 0
{
    println!("the number is even");
}
else{
    println!("the number is odd");
}
}


 fn main(){
    let number = 10;

    for i in 0..12{
        println!("{}",i);
    }hnnha
 }
 */
 fn main(){
    // array , maps and strings

    let sentence  = String::from("my name is aditya");
    let fist_word    = get_first_word(sentence);

     println!("first_word is: {}", fist_word);
 }

 fn get_first_word(sentence: String) -> String {
    let mut ans =String::from("");
    for char in sentence.chars(){

    ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
    }
 }
 return ans;
}
 */
fn main() {
    let sentence = String::from("my name is aditya");
    let first_word = get_first_word(sentence);
    println!("First word is: {}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut ans = String::from("");
    for char in sentence.chars() {
        if char == ' ' {
            break;
        }
        ans.push(char);
    }
    ans
}
