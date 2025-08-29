/* fn  main(){
    let n =10;;
    let mut a = 0;
    let mut b = 1;
     
     println!("fibonacci  up to {}",n);
    

 for _ in 0..n  {
   println!("{}",a);
   let temp =a+b;
   a=b;
   b= temp;
 }
} */

fn main()
{
    println!("{}", fib(10));
}
 fn fib(num:u32) ->u32{
    let mut first = 0;
    let mut second = 1;


 if num == 0 
 {
    return 0
 }
 if num ==1
 {
    return 1
 }
  for _ in 0..(num-2){
    let temp = second;
    second= first+second;
    first = temp;

  }
  return second;
}