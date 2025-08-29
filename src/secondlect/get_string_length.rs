fn get_string_length(s: &str) -> usize
{
    s.chars().count()
}
 
  fn main()
  {
    let my_string = String::from("hello,world");
    let length = get_string_length(&my_string);
    println!("the number of characters in the string is: {}", length);
  }
  