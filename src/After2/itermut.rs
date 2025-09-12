fn main() {
    let mut v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter_mut();

    /*  for val in v1_iter{
      *val = *val +1
    } */

    while let Some(val) = v1_iter.next() {
        println!("{}", val);
    }
    println!("{:?}", v1);
}
/*
/fn main() {
    let mut nums = vec![1, 2, 3];

    let iter = nums.iter_mut();

    for value in iter {
        *value = *value + 1;
    }





    println!("{:?}", nums);
}
 */
