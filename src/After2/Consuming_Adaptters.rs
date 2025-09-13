fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    //  let v1_iter = v1.into_iter(); //  uye use  ni kr skte hm 
    // ownership move ho gaya.

    let sum: i32 = v1_iter.sum();

    println!("Sum is : {}", sum);

    println!("{:?}", v1);
}
