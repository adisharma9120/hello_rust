/* fn get_string_length(s: &str) -> usize
{
    s.chars().count()
}
   
  fn main()
  {
    let my_string = String::from("hello,world");
    let length = get_string_length(&my_string);
    println!("the number of characters in the string is: {}", length);
  }
   */

  fn main()
  {
    let name = String::from("aditya");
    let len = get_str_len(name);
    println!("the length of the string is {}", len);

  }
  fn get_str_len(str:String)-> usize
  {
    return str.chars().count();
  }