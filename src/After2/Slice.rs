/* fn  main()
{
    let name = String::from("Aditya");

    let first_name = &name[0..5];
    let last_name= &name[5..];
    println!("{}, {}", first_name, last_name);

}


 // Write a function that takes string as an input and
 //  return the first word from it ......

fn main()
{
    let name = String::from("hello world");
    let ans = first_word(name);
    println!("ans is: {}", ans);
}
 fn first_word(str:String) ->String{
/*
Ye ek function definition hai:
Parameter: str:String → function ek String lega (ownership ke saath).
Return type: -> String → function ek String return karega.
str naam ka variable yaha ek local copy ban jaata hai (jo original name ka ownership le leta hai). */
    let mut ans = String::from("");
    for i in str.chars(){ //.chars() ek iterator banata hai jo string ke har character ko ek–ek karke deta hai.
                    // Example: "hello world".chars() → ['h', 'e', 'l', 'l', 'o', ' ', 'w', ...]
        if i == ' '{
          break;
        }
        ans.push_str(&i.to_string());
     }
     return ans;
 }





fn main() {
    let mut word = String::from("hello world");
    let word2 = &word[0..5];

    println!("{}", word2);
    
}
 */
//  Reference ka use khatam hone ke baad hi clear() call kar
fn main() {
    let mut word = String::from("hello world");
    {
        let word2 = &word[0..5];
        println!("{}", word2);  // yaha tak reference ka kaam ho gaya
    }
    word.clear();  // ab safe hai
}
