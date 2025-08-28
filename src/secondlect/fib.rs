fn  main(){
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
}