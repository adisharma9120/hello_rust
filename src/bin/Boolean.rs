fn main()
{
    let ans = is_even( 22);
    println!("{}",ans);

}
fn is_even(num: i32) -> bool{
    if num % 2 == 0{
        return true;
    }
    else {
        return  false;
    }

}