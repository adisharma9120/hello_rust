/* fn  main()
{
    let name = String::from("Aditya");

    let first_name = &name[0..5];
    let last_name= &name[5..];
    println!("{}, {}", first_name, last_name);

}

*/
 // Write a function that takes string as an input and 
 //  return the first word from it ......

 fn main()
 {
    let name = String::from("Hello World");
    let ans = first_word(name);
    let ans1 =  last_word(name);
        println!("Ans is: {}", ans);
 }
 fn first_word(str:String) ->String{
    let mut ans = String::from("");
    for i in str.chars(){
        if i == ' '{
          break;
        }   
        ans.push_str(&i.to_string());
     }
     return ans;
 }